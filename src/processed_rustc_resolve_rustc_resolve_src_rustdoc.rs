/* FP:rustdoc.rs-0001 */ use std::mem;
/* FP:rustdoc.rs-0002 */ use std::ops::Range;
/* FP:rustdoc.rs-0003 */ 
/* FP:rustdoc.rs-0004 */ use itertools::Itertools;
/* FP:rustdoc.rs-0005 */ use pulldown_cmark::{
/* FP:rustdoc.rs-0006 */     BrokenLink, BrokenLinkCallback, CowStr, Event, LinkType, Options, Parser, Tag,
/* FP:rustdoc.rs-0007 */ };
/* FP:rustdoc.rs-0008 */ use rustc_ast as ast;
/* FP:rustdoc.rs-0009 */ use crate::rustc_complete::attr::AttributeExt;
/* FP:rustdoc.rs-0010 */ use crate::rustc_complete::join_path_syms;
/* FP:rustdoc.rs-0011 */ use crate::rustc_complete::util::comments::beautify_doc_string;
/* FP:rustdoc.rs-0012 */ use crate::rustc_data_structures::fx::FxIndexMap;
/* FP:rustdoc.rs-0013 */ use crate::rustc_data_structures::unord::UnordSet;
/* FP:rustdoc.rs-0014 */ use crate::rustc_complete::ty::TyCtxt;
/* FP:rustdoc.rs-0015 */ use crate::rustc_complete::def_id::DefId;
/* FP:rustdoc.rs-0016 */ use crate::rustc_complete::source_map::SourceMap;
/* FP:rustdoc.rs-0017 */ use crate::rustc_complete::{DUMMY_SP, InnerSpan, Span, Symbol, sym};
/* FP:rustdoc.rs-0018 */ use thin_vec::ThinVec;
/* FP:rustdoc.rs-0019 */ use tracing::{debug, trace};
/* FP:rustdoc.rs-0020 */ 
/* FP:rustdoc.rs-0021 */ #[cfg(test)]
/* FP:rustdoc.rs-0023 */ 
/* FP:rustdoc.rs-0024 */ #[derive(Clone, Copy, PartialEq, Eq, Debug)]
/* FP:rustdoc.rs-0025 */ pub enum DocFragmentKind {
/* FP:rustdoc.rs-0026 */     /// A doc fragment created from a `///` or `//` doc comment.
/* FP:rustdoc.rs-0027 */     SugaredDoc,
/* FP:rustdoc.rs-0028 */     /// A doc fragment created from a "raw" `#[doc=""]` attribute.
/* FP:rustdoc.rs-0029 */     RawDoc,
/* FP:rustdoc.rs-0030 */ }
/* FP:rustdoc.rs-0031 */ 
/* FP:rustdoc.rs-0032 */ /// A portion of documentation, extracted from a `#[doc]` attribute.
/* FP:rustdoc.rs-0033 */ ///
/* FP:rustdoc.rs-0034 */ /// Each variant contains the line number within the complete doc-comment where the fragment
/* FP:rustdoc.rs-0035 */ /// starts, as well as the Span where the corresponding doc comment or attribute is located.
/* FP:rustdoc.rs-0036 */ ///
/* FP:rustdoc.rs-0037 */ /// Included files are kept separate from inline doc comments so that proper line-number
/* FP:rustdoc.rs-0038 */ /// information can be given when a doctest fails. Sugared doc comments and "raw" doc comments are
/* FP:rustdoc.rs-0039 */ /// kept separate because of issue #42760.
/* FP:rustdoc.rs-0040 */ #[derive(Clone, PartialEq, Eq, Debug)]
/* FP:rustdoc.rs-0041 */ pub struct DocFragment {
/* FP:rustdoc.rs-0042 */     pub span: Span,
/* FP:rustdoc.rs-0043 */     /// The item this doc-comment came from.
/* FP:rustdoc.rs-0044 */     /// Used to determine the scope in which doc links in this fragment are resolved.
/* FP:rustdoc.rs-0045 */     /// Typically filled for reexport docs when they are merged into the docs of the
/* FP:rustdoc.rs-0046 */     /// original reexported item.
/* FP:rustdoc.rs-0047 */     /// If the id is not filled, which happens for the original reexported item, then
/* FP:rustdoc.rs-0048 */     /// it has to be taken from somewhere else during doc link resolution.
/* FP:rustdoc.rs-0049 */     pub item_id: Option<DefId>,
/* FP:rustdoc.rs-0050 */     pub doc: Symbol,
/* FP:rustdoc.rs-0051 */     pub kind: DocFragmentKind,
/* FP:rustdoc.rs-0052 */     pub indent: usize,
/* FP:rustdoc.rs-0053 */     /// Because we tamper with the spans context, this information cannot be correctly retrieved
/* FP:rustdoc.rs-0054 */     /// later on. So instead, we compute it and store it here.
/* FP:rustdoc.rs-0055 */     pub from_expansion: bool,
/* FP:rustdoc.rs-0056 */ }
/* FP:rustdoc.rs-0057 */ 
/* FP:rustdoc.rs-0058 */ #[derive(Clone, Copy, Debug)]
/* FP:rustdoc.rs-0059 */ pub enum MalformedGenerics {
/* FP:rustdoc.rs-0060 */     /// This link has unbalanced angle brackets.
/* FP:rustdoc.rs-0061 */     ///
/* FP:rustdoc.rs-0062 */     /// For example, `Vec<T` should trigger this, as should `Vec<T>>`.
/* FP:rustdoc.rs-0063 */     UnbalancedAngleBrackets,
/* FP:rustdoc.rs-0064 */     /// The generics are not attached to a type.
/* FP:rustdoc.rs-0065 */     ///
/* FP:rustdoc.rs-0066 */     /// For example, `<T>` should trigger this.
/* FP:rustdoc.rs-0067 */     ///
/* FP:rustdoc.rs-0068 */     /// This is detected by checking if the path is empty after the generics are stripped.
/* FP:rustdoc.rs-0069 */     MissingType,
/* FP:rustdoc.rs-0070 */     /// The link uses fully-qualified syntax, which is currently unsupported.
/* FP:rustdoc.rs-0071 */     ///
/* FP:rustdoc.rs-0072 */     /// For example, `<Vec as IntoIterator>::into_iter` should trigger this.
/* FP:rustdoc.rs-0073 */     ///
/* FP:rustdoc.rs-0074 */     /// This is detected by checking if ` as ` (the keyword `as` with spaces around it) is inside
/* FP:rustdoc.rs-0075 */     /// angle brackets.
/* FP:rustdoc.rs-0076 */     HasFullyQualifiedSyntax,
/* FP:rustdoc.rs-0077 */     /// The link has an invalid path separator.
/* FP:rustdoc.rs-0078 */     ///
/* FP:rustdoc.rs-0079 */     /// For example, `Vec:<T>:new()` should trigger this. Note that `Vec:new()` will **not**
/* FP:rustdoc.rs-0080 */     /// trigger this because it has no generics and thus [`strip_generics_from_path`] will not be
/* FP:rustdoc.rs-0081 */     /// called.
/* FP:rustdoc.rs-0082 */     ///
/* FP:rustdoc.rs-0083 */     /// Note that this will also **not** be triggered if the invalid path separator is inside angle
/* FP:rustdoc.rs-0084 */     /// brackets because rustdoc mostly ignores what's inside angle brackets (except for
/* FP:rustdoc.rs-0085 */     /// [`HasFullyQualifiedSyntax`](MalformedGenerics::HasFullyQualifiedSyntax)).
/* FP:rustdoc.rs-0086 */     ///
/* FP:rustdoc.rs-0087 */     /// This is detected by checking if there is a colon followed by a non-colon in the link.
/* FP:rustdoc.rs-0088 */     InvalidPathSeparator,
/* FP:rustdoc.rs-0089 */     /// The link has too many angle brackets.
/* FP:rustdoc.rs-0090 */     ///
/* FP:rustdoc.rs-0091 */     /// For example, `Vec<<T>>` should trigger this.
/* FP:rustdoc.rs-0092 */     TooManyAngleBrackets,
/* FP:rustdoc.rs-0093 */     /// The link has empty angle brackets.
/* FP:rustdoc.rs-0094 */     ///
/* FP:rustdoc.rs-0095 */     /// For example, `Vec<>` should trigger this.
/* FP:rustdoc.rs-0096 */     EmptyAngleBrackets,
/* FP:rustdoc.rs-0097 */ }
/* FP:rustdoc.rs-0098 */ 
/* FP:rustdoc.rs-0099 */ /// Removes excess indentation on comments in order for the Markdown
/* FP:rustdoc.rs-0100 */ /// to be parsed correctly. This is necessary because the convention for
/* FP:rustdoc.rs-0101 */ /// writing documentation is to provide a space between the /// or // marker
/* FP:rustdoc.rs-0102 */ /// and the doc text, but Markdown is whitespace-sensitive. For example,
/* FP:rustdoc.rs-0103 */ /// a block of text with four-space indentation is parsed as a code block,
/* FP:rustdoc.rs-0104 */ /// so if we didn't unindent comments, these list items
/* FP:rustdoc.rs-0105 */ ///
/* FP:rustdoc.rs-0106 */ /// /// A list:
/* FP:rustdoc.rs-0107 */ /// ///
/* FP:rustdoc.rs-0108 */ /// ///    - Foo
/* FP:rustdoc.rs-0109 */ /// ///    - Bar
/* FP:rustdoc.rs-0110 */ ///
/* FP:rustdoc.rs-0111 */ /// would be parsed as if they were in a code block, which is likely not what the user intended.
/* FP:rustdoc.rs-0112 */ pub fn unindent_doc_fragments(docs: &mut [DocFragment]) {
/* FP:rustdoc.rs-0113 */     // `add` is used in case the most common sugared doc syntax is used ("/// "). The other
/* FP:rustdoc.rs-0114 */     // fragments kind's lines are never starting with a whitespace unless they are using some
/* FP:rustdoc.rs-0115 */     // markdown formatting requiring it. Therefore, if the doc block have a mix between the two,
/* FP:rustdoc.rs-0116 */     // we need to take into account the fact that the minimum indent minus one (to take this
/* FP:rustdoc.rs-0117 */     // whitespace into account).
/* FP:rustdoc.rs-0118 */     //
/* FP:rustdoc.rs-0119 */     // For example:
/* FP:rustdoc.rs-0120 */     //
/* FP:rustdoc.rs-0121 */     // /// hello!
/* FP:rustdoc.rs-0122 */     // #[doc = "another"]
/* FP:rustdoc.rs-0123 */     //
/* FP:rustdoc.rs-0124 */     // In this case, you want "hello! another" and not "hello!  another".
/* FP:rustdoc.rs-0125 */     let add = if docs.windows(2).any(|arr| arr[0].kind != arr[1].kind)
/* FP:rustdoc.rs-0126 */         && docs.iter().any(|d| d.kind == DocFragmentKind::SugaredDoc)
/* FP:rustdoc.rs-0127 */     {
/* FP:rustdoc.rs-0128 */         // In case we have a mix of sugared doc comments and "raw" ones, we want the sugared one to
/* FP:rustdoc.rs-0129 */         // "decide" how much the minimum indent will be.
/* FP:rustdoc.rs-0130 */         1
/* FP:rustdoc.rs-0131 */     } else {
/* FP:rustdoc.rs-0132 */         0
/* FP:rustdoc.rs-0133 */     };
/* FP:rustdoc.rs-0134 */ 
/* FP:rustdoc.rs-0135 */     // `min_indent` is used to know how much whitespaces from the start of each lines must be
/* FP:rustdoc.rs-0136 */     // removed. Example:
/* FP:rustdoc.rs-0137 */     //
/* FP:rustdoc.rs-0138 */     // ///     hello!
/* FP:rustdoc.rs-0139 */     // #[doc = "another"]
/* FP:rustdoc.rs-0140 */     //
/* FP:rustdoc.rs-0141 */     // In here, the `min_indent` is 1 (because non-sugared fragment are always counted with minimum
/* FP:rustdoc.rs-0142 */     // 1 whitespace), meaning that "hello!" will be considered a codeblock because it starts with 4
/* FP:rustdoc.rs-0143 */     // (5 - 1) whitespaces.
/* FP:rustdoc.rs-0144 */     let Some(min_indent) = docs
/* FP:rustdoc.rs-0145 */         .iter()
/* FP:rustdoc.rs-0146 */         .map(|fragment| {
/* FP:rustdoc.rs-0147 */             fragment
/* FP:rustdoc.rs-0148 */                 .doc
/* FP:rustdoc.rs-0149 */                 .as_str()
/* FP:rustdoc.rs-0150 */                 .lines()
/* FP:rustdoc.rs-0151 */                 .filter(|line| line.chars().any(|c| !c.is_whitespace()))
/* FP:rustdoc.rs-0152 */                 .map(|line| {
/* FP:rustdoc.rs-0153 */                     // Compare against either space or tab, ignoring whether they are
/* FP:rustdoc.rs-0154 */                     // mixed or not.
/* FP:rustdoc.rs-0155 */                     let whitespace = line.chars().take_while(|c| *c == ' ' || *c == '\t').count();
/* FP:rustdoc.rs-0156 */                     whitespace
/* FP:rustdoc.rs-0157 */                         + (if fragment.kind == DocFragmentKind::SugaredDoc { 0 } else { add })
/* FP:rustdoc.rs-0158 */                 })
/* FP:rustdoc.rs-0159 */                 .min()
/* FP:rustdoc.rs-0160 */                 .unwrap_or(usize::MAX)
/* FP:rustdoc.rs-0161 */         })
/* FP:rustdoc.rs-0162 */         .min()
/* FP:rustdoc.rs-0163 */     else {
/* FP:rustdoc.rs-0164 */         return;
/* FP:rustdoc.rs-0165 */     };
/* FP:rustdoc.rs-0166 */ 
/* FP:rustdoc.rs-0167 */     for fragment in docs {
/* FP:rustdoc.rs-0168 */         if fragment.doc == sym::empty {
/* FP:rustdoc.rs-0169 */             continue;
/* FP:rustdoc.rs-0170 */         }
/* FP:rustdoc.rs-0171 */ 
/* FP:rustdoc.rs-0172 */         let indent = if fragment.kind != DocFragmentKind::SugaredDoc && min_indent > 0 {
/* FP:rustdoc.rs-0173 */             min_indent - add
/* FP:rustdoc.rs-0174 */         } else {
/* FP:rustdoc.rs-0175 */             min_indent
/* FP:rustdoc.rs-0176 */         };
/* FP:rustdoc.rs-0177 */ 
/* FP:rustdoc.rs-0178 */         fragment.indent = indent;
/* FP:rustdoc.rs-0179 */     }
/* FP:rustdoc.rs-0180 */ }
/* FP:rustdoc.rs-0181 */ 
/* FP:rustdoc.rs-0182 */ /// The goal of this function is to apply the `DocFragment` transformation that is required when
/* FP:rustdoc.rs-0183 */ /// transforming into the final Markdown, which is applying the computed indent to each line in
/* FP:rustdoc.rs-0184 */ /// each doc fragment (a `DocFragment` can contain multiple lines in case of `#[doc = ""]`).
/* FP:rustdoc.rs-0185 */ ///
/* FP:rustdoc.rs-0186 */ /// Note: remove the trailing newline where appropriate
/* FP:rustdoc.rs-0187 */ pub fn add_doc_fragment(out: &mut String, frag: &DocFragment) {
/* FP:rustdoc.rs-0188 */     if frag.doc == sym::empty {
/* FP:rustdoc.rs-0189 */         out.push('\n');
/* FP:rustdoc.rs-0190 */         return;
/* FP:rustdoc.rs-0191 */     }
/* FP:rustdoc.rs-0192 */     let s = frag.doc.as_str();
/* FP:rustdoc.rs-0193 */     let mut iter = s.lines();
/* FP:rustdoc.rs-0194 */ 
/* FP:rustdoc.rs-0195 */     while let Some(line) = iter.next() {
/* FP:rustdoc.rs-0196 */         if line.chars().any(|c| !c.is_whitespace()) {
/* FP:rustdoc.rs-0197 */             assert!(line.len() >= frag.indent);
/* FP:rustdoc.rs-0198 */             out.push_str(&line[frag.indent..]);
/* FP:rustdoc.rs-0199 */         } else {
/* FP:rustdoc.rs-0200 */             out.push_str(line);
/* FP:rustdoc.rs-0201 */         }
/* FP:rustdoc.rs-0202 */         out.push('\n');
/* FP:rustdoc.rs-0203 */     }
/* FP:rustdoc.rs-0204 */ }
/* FP:rustdoc.rs-0205 */ 
/* FP:rustdoc.rs-0206 */ pub fn attrs_to_doc_fragments<'a, A: AttributeExt + Clone + 'a>(
/* FP:rustdoc.rs-0207 */     attrs: impl Iterator<Item = (&'a A, Option<DefId>)>,
/* FP:rustdoc.rs-0208 */     doc_only: bool,
/* FP:rustdoc.rs-0209 */ ) -> (Vec<DocFragment>, ThinVec<A>) {
/* FP:rustdoc.rs-0210 */     let (min_size, max_size) = attrs.size_hint();
/* FP:rustdoc.rs-0211 */     let size_hint = max_size.unwrap_or(min_size);
/* FP:rustdoc.rs-0212 */     let mut doc_fragments = Vec::with_capacity(size_hint);
/* FP:rustdoc.rs-0213 */     let mut other_attrs = ThinVec::<A>::with_capacity(if doc_only { 0 } else { size_hint });
/* FP:rustdoc.rs-0214 */     for (attr, item_id) in attrs {
/* FP:rustdoc.rs-0215 */         if let Some((doc_str, comment_kind)) = attr.doc_str_and_comment_kind() {
/* FP:rustdoc.rs-0216 */             let doc = beautify_doc_string(doc_str, comment_kind);
/* FP:rustdoc.rs-0217 */             let (span, kind, from_expansion) = if attr.is_doc_comment() {
/* FP:rustdoc.rs-0218 */                 let span = attr.span();
/* FP:rustdoc.rs-0219 */                 (span, DocFragmentKind::SugaredDoc, span.from_expansion())
/* FP:rustdoc.rs-0220 */             } else {
/* FP:rustdoc.rs-0221 */                 let attr_span = attr.span();
/* FP:rustdoc.rs-0222 */                 let (span, from_expansion) = match attr.value_span() {
/* FP:rustdoc.rs-0223 */                     Some(sp) => (sp.with_ctxt(attr_span.ctxt()), sp.from_expansion()),
/* FP:rustdoc.rs-0224 */                     None => (attr_span, attr_span.from_expansion()),
/* FP:rustdoc.rs-0225 */                 };
/* FP:rustdoc.rs-0226 */                 (span, DocFragmentKind::RawDoc, from_expansion)
/* FP:rustdoc.rs-0227 */             };
/* FP:rustdoc.rs-0228 */             let fragment = DocFragment { span, doc, kind, item_id, indent: 0, from_expansion };
/* FP:rustdoc.rs-0229 */             doc_fragments.push(fragment);
/* FP:rustdoc.rs-0230 */         } else if !doc_only {
/* FP:rustdoc.rs-0231 */             other_attrs.push(attr.clone());
/* FP:rustdoc.rs-0232 */         }
/* FP:rustdoc.rs-0233 */     }
/* FP:rustdoc.rs-0234 */ 
/* FP:rustdoc.rs-0235 */     doc_fragments.shrink_to_fit();
/* FP:rustdoc.rs-0236 */     other_attrs.shrink_to_fit();
/* FP:rustdoc.rs-0237 */ 
/* FP:rustdoc.rs-0238 */     unindent_doc_fragments(&mut doc_fragments);
/* FP:rustdoc.rs-0239 */ 
/* FP:rustdoc.rs-0240 */     (doc_fragments, other_attrs)
/* FP:rustdoc.rs-0241 */ }
/* FP:rustdoc.rs-0242 */ 
/* FP:rustdoc.rs-0243 */ /// Return the doc-comments on this item, grouped by the module they came from.
/* FP:rustdoc.rs-0244 */ /// The module can be different if this is a re-export with added documentation.
/* FP:rustdoc.rs-0245 */ ///
/* FP:rustdoc.rs-0246 */ /// The last newline is not trimmed so the produced strings are reusable between
/* FP:rustdoc.rs-0247 */ /// early and late doc link resolution regardless of their position.
/* FP:rustdoc.rs-0248 */ pub fn prepare_to_doc_link_resolution(
/* FP:rustdoc.rs-0249 */     doc_fragments: &[DocFragment],
/* FP:rustdoc.rs-0250 */ ) -> FxIndexMap<Option<DefId>, String> {
/* FP:rustdoc.rs-0251 */     let mut res = FxIndexMap::default();
/* FP:rustdoc.rs-0252 */     for fragment in doc_fragments {
/* FP:rustdoc.rs-0253 */         let out_str = res.entry(fragment.item_id).or_default();
/* FP:rustdoc.rs-0254 */         add_doc_fragment(out_str, fragment);
/* FP:rustdoc.rs-0255 */     }
/* FP:rustdoc.rs-0256 */     res
/* FP:rustdoc.rs-0257 */ }
/* FP:rustdoc.rs-0258 */ 
/* FP:rustdoc.rs-0259 */ /// Options for rendering Markdown in the main body of documentation.
/* FP:rustdoc.rs-0260 */ pub fn main_body_opts() -> Options {
/* FP:rustdoc.rs-0261 */     Options::ENABLE_TABLES
/* FP:rustdoc.rs-0262 */         | Options::ENABLE_FOOTNOTES
/* FP:rustdoc.rs-0263 */         | Options::ENABLE_STRIKETHROUGH
/* FP:rustdoc.rs-0264 */         | Options::ENABLE_TASKLISTS
/* FP:rustdoc.rs-0265 */         | Options::ENABLE_SMART_PUNCTUATION
/* FP:rustdoc.rs-0266 */ }
/* FP:rustdoc.rs-0267 */ 
/* FP:rustdoc.rs-0268 */ fn strip_generics_from_path_segment(segment: Vec<char>) -> Result<Symbol, MalformedGenerics> {
/* FP:rustdoc.rs-0269 */     let mut stripped_segment = String::new();
/* FP:rustdoc.rs-0270 */     let mut param_depth = 0;
/* FP:rustdoc.rs-0271 */ 
/* FP:rustdoc.rs-0272 */     let mut latest_generics_chunk = String::new();
/* FP:rustdoc.rs-0273 */ 
/* FP:rustdoc.rs-0274 */     for c in segment {
/* FP:rustdoc.rs-0275 */         if c == '<' {
/* FP:rustdoc.rs-0276 */             param_depth += 1;
/* FP:rustdoc.rs-0277 */             latest_generics_chunk.clear();
/* FP:rustdoc.rs-0278 */         } else if c == '>' {
/* FP:rustdoc.rs-0279 */             param_depth -= 1;
/* FP:rustdoc.rs-0280 */             if latest_generics_chunk.contains(" as ") {
/* FP:rustdoc.rs-0281 */                 // The segment tries to use fully-qualified syntax, which is currently unsupported.
/* FP:rustdoc.rs-0282 */                 // Give a helpful error message instead of completely ignoring the angle brackets.
/* FP:rustdoc.rs-0283 */                 return Err(MalformedGenerics::HasFullyQualifiedSyntax);
/* FP:rustdoc.rs-0284 */             }
/* FP:rustdoc.rs-0285 */         } else if param_depth == 0 {
/* FP:rustdoc.rs-0286 */             stripped_segment.push(c);
/* FP:rustdoc.rs-0287 */         } else {
/* FP:rustdoc.rs-0288 */             latest_generics_chunk.push(c);
/* FP:rustdoc.rs-0289 */         }
/* FP:rustdoc.rs-0290 */     }
/* FP:rustdoc.rs-0291 */ 
/* FP:rustdoc.rs-0292 */     if param_depth == 0 {
/* FP:rustdoc.rs-0293 */         Ok(Symbol::intern(&stripped_segment))
/* FP:rustdoc.rs-0294 */     } else {
/* FP:rustdoc.rs-0295 */         // The segment has unbalanced angle brackets, e.g. `Vec<T` or `Vec<T>>`
/* FP:rustdoc.rs-0296 */         Err(MalformedGenerics::UnbalancedAngleBrackets)
/* FP:rustdoc.rs-0297 */     }
/* FP:rustdoc.rs-0298 */ }
/* FP:rustdoc.rs-0299 */ 
/* FP:rustdoc.rs-0300 */ pub fn strip_generics_from_path(path_str: &str) -> Result<Box<str>, MalformedGenerics> {
/* FP:rustdoc.rs-0301 */     if !path_str.contains(['<', '>']) {
/* FP:rustdoc.rs-0302 */         return Ok(path_str.into());
/* FP:rustdoc.rs-0303 */     }
/* FP:rustdoc.rs-0304 */     let mut stripped_segments = vec![];
/* FP:rustdoc.rs-0305 */     let mut path = path_str.chars().peekable();
/* FP:rustdoc.rs-0306 */     let mut segment = Vec::new();
/* FP:rustdoc.rs-0307 */ 
/* FP:rustdoc.rs-0308 */     while let Some(chr) = path.next() {
/* FP:rustdoc.rs-0309 */         match chr {
/* FP:rustdoc.rs-0310 */             ':' => {
/* FP:rustdoc.rs-0311 */                 if path.next_if_eq(&':').is_some() {
/* FP:rustdoc.rs-0312 */                     let stripped_segment =
/* FP:rustdoc.rs-0313 */                         strip_generics_from_path_segment(mem::take(&mut segment))?;
/* FP:rustdoc.rs-0314 */                     if !stripped_segment.is_empty() {
/* FP:rustdoc.rs-0315 */                         stripped_segments.push(stripped_segment);
/* FP:rustdoc.rs-0316 */                     }
/* FP:rustdoc.rs-0317 */                 } else {
/* FP:rustdoc.rs-0318 */                     return Err(MalformedGenerics::InvalidPathSeparator);
/* FP:rustdoc.rs-0319 */                 }
/* FP:rustdoc.rs-0320 */             }
/* FP:rustdoc.rs-0321 */             '<' => {
/* FP:rustdoc.rs-0322 */                 segment.push(chr);
/* FP:rustdoc.rs-0323 */ 
/* FP:rustdoc.rs-0324 */                 match path.next() {
/* FP:rustdoc.rs-0325 */                     Some('<') => {
/* FP:rustdoc.rs-0326 */                         return Err(MalformedGenerics::TooManyAngleBrackets);
/* FP:rustdoc.rs-0327 */                     }
/* FP:rustdoc.rs-0328 */                     Some('>') => {
/* FP:rustdoc.rs-0329 */                         return Err(MalformedGenerics::EmptyAngleBrackets);
/* FP:rustdoc.rs-0330 */                     }
/* FP:rustdoc.rs-0331 */                     Some(chr) => {
/* FP:rustdoc.rs-0332 */                         segment.push(chr);
/* FP:rustdoc.rs-0333 */ 
/* FP:rustdoc.rs-0334 */                         while let Some(chr) = path.next_if(|c| *c != '>') {
/* FP:rustdoc.rs-0335 */                             segment.push(chr);
/* FP:rustdoc.rs-0336 */                         }
/* FP:rustdoc.rs-0337 */                     }
/* FP:rustdoc.rs-0338 */                     None => break,
/* FP:rustdoc.rs-0339 */                 }
/* FP:rustdoc.rs-0340 */             }
/* FP:rustdoc.rs-0341 */             _ => segment.push(chr),
/* FP:rustdoc.rs-0342 */         }
/* FP:rustdoc.rs-0343 */         trace!("raw segment: {:?}", segment);
/* FP:rustdoc.rs-0344 */     }
/* FP:rustdoc.rs-0345 */ 
/* FP:rustdoc.rs-0346 */     if !segment.is_empty() {
/* FP:rustdoc.rs-0347 */         let stripped_segment = strip_generics_from_path_segment(segment)?;
/* FP:rustdoc.rs-0348 */         if !stripped_segment.is_empty() {
/* FP:rustdoc.rs-0349 */             stripped_segments.push(stripped_segment);
/* FP:rustdoc.rs-0350 */         }
/* FP:rustdoc.rs-0351 */     }
/* FP:rustdoc.rs-0352 */ 
/* FP:rustdoc.rs-0353 */     debug!("path_str: {path_str:?}\nstripped segments: {stripped_segments:?}");
/* FP:rustdoc.rs-0354 */ 
/* FP:rustdoc.rs-0355 */     if !stripped_segments.is_empty() {
/* FP:rustdoc.rs-0356 */         let stripped_path = join_path_syms(stripped_segments);
/* FP:rustdoc.rs-0357 */         Ok(stripped_path.into())
/* FP:rustdoc.rs-0358 */     } else {
/* FP:rustdoc.rs-0359 */         Err(MalformedGenerics::MissingType)
/* FP:rustdoc.rs-0360 */     }
/* FP:rustdoc.rs-0361 */ }
/* FP:rustdoc.rs-0362 */ 
/* FP:rustdoc.rs-0363 */ /// Returns whether the first doc-comment is an inner attribute.
/* FP:rustdoc.rs-0364 */ ///
/* FP:rustdoc.rs-0365 */ /// If there are no doc-comments, return true.
/* FP:rustdoc.rs-0366 */ /// FIXME(#78591): Support both inner and outer attributes on the same item.
/* FP:rustdoc.rs-0367 */ pub fn inner_docs(attrs: &[impl AttributeExt]) -> bool {
/* FP:rustdoc.rs-0368 */     for attr in attrs {
/* FP:rustdoc.rs-0369 */         if let Some(attr_style) = attr.doc_resolution_scope() {
/* FP:rustdoc.rs-0370 */             return attr_style == ast::AttrStyle::Inner;
/* FP:rustdoc.rs-0371 */         }
/* FP:rustdoc.rs-0372 */     }
/* FP:rustdoc.rs-0373 */     true
/* FP:rustdoc.rs-0374 */ }
/* FP:rustdoc.rs-0375 */ 
/* FP:rustdoc.rs-0376 */ /// Has `#[rustc_doc_primitive]` or `#[doc(keyword)]` or `#[doc(attribute)]`.
/* FP:rustdoc.rs-0377 */ pub fn has_primitive_or_keyword_or_attribute_docs(attrs: &[impl AttributeExt]) -> bool {
/* FP:rustdoc.rs-0378 */     for attr in attrs {
/* FP:rustdoc.rs-0379 */         if attr.has_name(sym::rustc_doc_primitive) {
/* FP:rustdoc.rs-0380 */             return true;
/* FP:rustdoc.rs-0381 */         } else if attr.has_name(sym::doc)
/* FP:rustdoc.rs-0382 */             && let Some(items) = attr.meta_item_list()
/* FP:rustdoc.rs-0383 */         {
/* FP:rustdoc.rs-0384 */             for item in items {
/* FP:rustdoc.rs-0385 */                 if item.has_name(sym::keyword) || item.has_name(sym::attribute) {
/* FP:rustdoc.rs-0386 */                     return true;
/* FP:rustdoc.rs-0387 */                 }
/* FP:rustdoc.rs-0388 */             }
/* FP:rustdoc.rs-0389 */         }
/* FP:rustdoc.rs-0390 */     }
/* FP:rustdoc.rs-0391 */     false
/* FP:rustdoc.rs-0392 */ }
/* FP:rustdoc.rs-0393 */ 
/* FP:rustdoc.rs-0394 */ /// Simplified version of the corresponding function in rustdoc.
/* FP:rustdoc.rs-0395 */ /// If the rustdoc version returns a successful result, this function must return the same result.
/* FP:rustdoc.rs-0396 */ /// Otherwise this function may return anything.
/* FP:rustdoc.rs-0397 */ fn preprocess_link(link: &str) -> Box<str> {
/* FP:rustdoc.rs-0398 */     let link = link.replace('`', "");
/* FP:rustdoc.rs-0399 */     let link = link.split('#').next().unwrap();
/* FP:rustdoc.rs-0400 */     let link = link.trim();
/* FP:rustdoc.rs-0401 */     let link = link.rsplit('@').next().unwrap();
/* FP:rustdoc.rs-0402 */     let link = link.strip_suffix("()").unwrap_or(link);
/* FP:rustdoc.rs-0403 */     let link = link.strip_suffix("{}").unwrap_or(link);
/* FP:rustdoc.rs-0404 */     let link = link.strip_suffix("[]").unwrap_or(link);
/* FP:rustdoc.rs-0405 */     let link = if link != "!" { link.strip_suffix('!').unwrap_or(link) } else { link };
/* FP:rustdoc.rs-0406 */     let link = link.trim();
/* FP:rustdoc.rs-0407 */     strip_generics_from_path(link).unwrap_or_else(|_| link.into())
/* FP:rustdoc.rs-0408 */ }
/* FP:rustdoc.rs-0409 */ 
/* FP:rustdoc.rs-0410 */ /// Keep inline and reference links `[]`,
/* FP:rustdoc.rs-0411 */ /// but skip autolinks `<>` which we never consider to be intra-doc links.
/* FP:rustdoc.rs-0412 */ pub fn may_be_doc_link(link_type: LinkType) -> bool {
/* FP:rustdoc.rs-0413 */     match link_type {
/* FP:rustdoc.rs-0414 */         LinkType::Inline
/* FP:rustdoc.rs-0415 */         | LinkType::Reference
/* FP:rustdoc.rs-0416 */         | LinkType::ReferenceUnknown
/* FP:rustdoc.rs-0417 */         | LinkType::Collapsed
/* FP:rustdoc.rs-0418 */         | LinkType::CollapsedUnknown
/* FP:rustdoc.rs-0419 */         | LinkType::Shortcut
/* FP:rustdoc.rs-0420 */         | LinkType::ShortcutUnknown => true,
/* FP:rustdoc.rs-0421 */         LinkType::Autolink | LinkType::Email => false,
/* FP:rustdoc.rs-0422 */     }
/* FP:rustdoc.rs-0423 */ }
/* FP:rustdoc.rs-0424 */ 
/* FP:rustdoc.rs-0425 */ /// Simplified version of `preprocessed_markdown_links` from rustdoc.
/* FP:rustdoc.rs-0426 */ /// Must return at least the same links as it, but may add some more links on top of that.
/* FP:rustdoc.rs-0427 */ pub(crate) fn attrs_to_preprocessed_links<A: AttributeExt + Clone>(attrs: &[A]) -> Vec<Box<str>> {
/* FP:rustdoc.rs-0428 */     let (doc_fragments, _) = attrs_to_doc_fragments(attrs.iter().map(|attr| (attr, None)), true);
/* FP:rustdoc.rs-0429 */     let doc = prepare_to_doc_link_resolution(&doc_fragments).into_values().next().unwrap();
/* FP:rustdoc.rs-0430 */ 
/* FP:rustdoc.rs-0431 */     parse_links(&doc)
/* FP:rustdoc.rs-0432 */ }
/* FP:rustdoc.rs-0433 */ 
/* FP:rustdoc.rs-0434 */ /// Similar version of `markdown_links` from rustdoc.
/* FP:rustdoc.rs-0435 */ /// This will collect destination links and display text if exists.
/* FP:rustdoc.rs-0436 */ fn parse_links<'md>(doc: &'md str) -> Vec<Box<str>> {
/* FP:rustdoc.rs-0437 */     let mut broken_link_callback = |link: BrokenLink<'md>| Some((link.reference, "".into()));
/* FP:rustdoc.rs-0438 */     let mut event_iter = Parser::new_with_broken_link_callback(
/* FP:rustdoc.rs-0439 */         doc,
/* FP:rustdoc.rs-0440 */         main_body_opts(),
/* FP:rustdoc.rs-0441 */         Some(&mut broken_link_callback),
/* FP:rustdoc.rs-0442 */     );
/* FP:rustdoc.rs-0443 */     let mut links = Vec::new();
/* FP:rustdoc.rs-0444 */ 
/* FP:rustdoc.rs-0445 */     let mut refids = UnordSet::default();
/* FP:rustdoc.rs-0446 */ 
/* FP:rustdoc.rs-0447 */     while let Some(event) = event_iter.next() {
/* FP:rustdoc.rs-0448 */         match event {
/* FP:rustdoc.rs-0449 */             Event::Start(Tag::Link { link_type, dest_url, title: _, id })
/* FP:rustdoc.rs-0450 */                 if may_be_doc_link(link_type) =>
/* FP:rustdoc.rs-0451 */             {
/* FP:rustdoc.rs-0452 */                 if matches!(
/* FP:rustdoc.rs-0453 */                     link_type,
/* FP:rustdoc.rs-0454 */                     LinkType::Inline
/* FP:rustdoc.rs-0455 */                         | LinkType::ReferenceUnknown
/* FP:rustdoc.rs-0456 */                         | LinkType::Reference
/* FP:rustdoc.rs-0457 */                         | LinkType::Shortcut
/* FP:rustdoc.rs-0458 */                         | LinkType::ShortcutUnknown
/* FP:rustdoc.rs-0459 */                 ) {
/* FP:rustdoc.rs-0460 */                     if let Some(display_text) = collect_link_data(&mut event_iter) {
/* FP:rustdoc.rs-0461 */                         links.push(display_text);
/* FP:rustdoc.rs-0462 */                     }
/* FP:rustdoc.rs-0463 */                 }
/* FP:rustdoc.rs-0464 */                 if matches!(
/* FP:rustdoc.rs-0465 */                     link_type,
/* FP:rustdoc.rs-0466 */                     LinkType::Reference | LinkType::Shortcut | LinkType::Collapsed
/* FP:rustdoc.rs-0467 */                 ) {
/* FP:rustdoc.rs-0468 */                     refids.insert(id);
/* FP:rustdoc.rs-0469 */                 }
/* FP:rustdoc.rs-0470 */ 
/* FP:rustdoc.rs-0471 */                 links.push(preprocess_link(&dest_url));
/* FP:rustdoc.rs-0472 */             }
/* FP:rustdoc.rs-0473 */             _ => {}
/* FP:rustdoc.rs-0474 */         }
/* FP:rustdoc.rs-0475 */     }
/* FP:rustdoc.rs-0476 */ 
/* FP:rustdoc.rs-0477 */     for (label, refdef) in event_iter.reference_definitions().iter().sorted_by_key(|x| x.0) {
/* FP:rustdoc.rs-0478 */         if !refids.contains(label) {
/* FP:rustdoc.rs-0479 */             links.push(preprocess_link(&refdef.dest));
/* FP:rustdoc.rs-0480 */         }
/* FP:rustdoc.rs-0481 */     }
/* FP:rustdoc.rs-0482 */ 
/* FP:rustdoc.rs-0483 */     links
/* FP:rustdoc.rs-0484 */ }
/* FP:rustdoc.rs-0485 */ 
/* FP:rustdoc.rs-0486 */ /// Collects additional data of link.
/* FP:rustdoc.rs-0487 */ fn collect_link_data<'input, F: BrokenLinkCallback<'input>>(
/* FP:rustdoc.rs-0488 */     event_iter: &mut Parser<'input, F>,
/* FP:rustdoc.rs-0489 */ ) -> Option<Box<str>> {
/* FP:rustdoc.rs-0490 */     let mut display_text: Option<String> = None;
/* FP:rustdoc.rs-0491 */     let mut append_text = |text: CowStr<'_>| {
/* FP:rustdoc.rs-0492 */         if let Some(display_text) = &mut display_text {
/* FP:rustdoc.rs-0493 */             display_text.push_str(&text);
/* FP:rustdoc.rs-0494 */         } else {
/* FP:rustdoc.rs-0495 */             display_text = Some(text.to_string());
/* FP:rustdoc.rs-0496 */         }
/* FP:rustdoc.rs-0497 */     };
/* FP:rustdoc.rs-0498 */ 
/* FP:rustdoc.rs-0499 */     while let Some(event) = event_iter.next() {
/* FP:rustdoc.rs-0500 */         match event {
/* FP:rustdoc.rs-0501 */             Event::Text(text) => {
/* FP:rustdoc.rs-0502 */                 append_text(text);
/* FP:rustdoc.rs-0503 */             }
/* FP:rustdoc.rs-0504 */             Event::Code(code) => {
/* FP:rustdoc.rs-0505 */                 append_text(code);
/* FP:rustdoc.rs-0506 */             }
/* FP:rustdoc.rs-0507 */             Event::End(_) => {
/* FP:rustdoc.rs-0508 */                 break;
/* FP:rustdoc.rs-0509 */             }
/* FP:rustdoc.rs-0510 */             _ => {}
/* FP:rustdoc.rs-0511 */         }
/* FP:rustdoc.rs-0512 */     }
/* FP:rustdoc.rs-0513 */ 
/* FP:rustdoc.rs-0514 */     display_text.map(String::into_boxed_str)
/* FP:rustdoc.rs-0515 */ }
/* FP:rustdoc.rs-0516 */ 
/* FP:rustdoc.rs-0517 */ /// Returns a span encompassing all the document fragments.
/* FP:rustdoc.rs-0518 */ pub fn span_of_fragments(fragments: &[DocFragment]) -> Option<Span> {
/* FP:rustdoc.rs-0519 */     let (first_fragment, last_fragment) = match fragments {
/* FP:rustdoc.rs-0520 */         [] => return None,
/* FP:rustdoc.rs-0521 */         [first, .., last] => (first, last),
/* FP:rustdoc.rs-0522 */         [first] => (first, first),
/* FP:rustdoc.rs-0523 */     };
/* FP:rustdoc.rs-0524 */     if first_fragment.span == DUMMY_SP {
/* FP:rustdoc.rs-0525 */         return None;
/* FP:rustdoc.rs-0526 */     }
/* FP:rustdoc.rs-0527 */     Some(first_fragment.span.to(last_fragment.span))
/* FP:rustdoc.rs-0528 */ }
/* FP:rustdoc.rs-0529 */ 
/* FP:rustdoc.rs-0530 */ /// Attempts to match a range of bytes from parsed markdown to a `Span` in the source code.
/* FP:rustdoc.rs-0531 */ ///
/* FP:rustdoc.rs-0532 */ /// This method does not always work, because markdown bytes don't necessarily match source bytes,
/* FP:rustdoc.rs-0533 */ /// like if escapes are used in the string. In this case, it returns `None`.
/* FP:rustdoc.rs-0534 */ ///
/* FP:rustdoc.rs-0535 */ /// `markdown` is typically the entire documentation for an item,
/* FP:rustdoc.rs-0536 */ /// after combining fragments.
/* FP:rustdoc.rs-0537 */ ///
/* FP:rustdoc.rs-0538 */ /// This method will return `Some` only if one of the following is true:
/* FP:rustdoc.rs-0539 */ ///
/* FP:rustdoc.rs-0540 */ /// - The doc is made entirely from sugared doc comments, which cannot contain escapes
/* FP:rustdoc.rs-0541 */ /// - The doc is entirely from a single doc fragment with a string literal exactly equal to
/* FP:rustdoc.rs-0542 */ ///   `markdown`.
/* FP:rustdoc.rs-0543 */ /// - The doc comes from `include_str!`
/* FP:rustdoc.rs-0544 */ /// - The doc includes exactly one substring matching `markdown[md_range]` which is contained in a
/* FP:rustdoc.rs-0545 */ ///   single doc fragment.
/* FP:rustdoc.rs-0546 */ ///
/* FP:rustdoc.rs-0547 */ /// This function is defined in the compiler so it can be used by both `rustdoc` and `clippy`.
/* FP:rustdoc.rs-0548 */ ///
/* FP:rustdoc.rs-0549 */ /// It returns a tuple containing a span encompassing all the document fragments and a boolean that
/* FP:rustdoc.rs-0550 */ /// is `true` if any of the *matched* fragments are from a macro expansion.
/* FP:rustdoc.rs-0551 */ pub fn source_span_for_markdown_range(
/* FP:rustdoc.rs-0552 */     tcx: TyCtxt<'_>,
/* FP:rustdoc.rs-0553 */     markdown: &str,
/* FP:rustdoc.rs-0554 */     md_range: &Range<usize>,
/* FP:rustdoc.rs-0555 */     fragments: &[DocFragment],
/* FP:rustdoc.rs-0556 */ ) -> Option<(Span, bool)> {
/* FP:rustdoc.rs-0557 */     let map = tcx.sess.source_map();
/* FP:rustdoc.rs-0558 */     source_span_for_markdown_range_inner(map, markdown, md_range, fragments)
/* FP:rustdoc.rs-0559 */ }
/* FP:rustdoc.rs-0560 */ 
/* FP:rustdoc.rs-0561 */ // inner function used for unit testing
/* FP:rustdoc.rs-0562 */ pub fn source_span_for_markdown_range_inner(
/* FP:rustdoc.rs-0563 */     map: &SourceMap,
/* FP:rustdoc.rs-0564 */     markdown: &str,
/* FP:rustdoc.rs-0565 */     md_range: &Range<usize>,
/* FP:rustdoc.rs-0566 */     fragments: &[DocFragment],
/* FP:rustdoc.rs-0567 */ ) -> Option<(Span, bool)> {
/* FP:rustdoc.rs-0568 */     use crate::rustc_complete::BytePos;
/* FP:rustdoc.rs-0569 */ 
/* FP:rustdoc.rs-0570 */     if let &[fragment] = &fragments
/* FP:rustdoc.rs-0571 */         && fragment.kind == DocFragmentKind::RawDoc
/* FP:rustdoc.rs-0572 */         && let Ok(snippet) = map.span_to_snippet(fragment.span)
/* FP:rustdoc.rs-0573 */         && snippet.trim_end() == markdown.trim_end()
/* FP:rustdoc.rs-0574 */         && let Ok(md_range_lo) = u32::try_from(md_range.start)
/* FP:rustdoc.rs-0575 */         && let Ok(md_range_hi) = u32::try_from(md_range.end)
/* FP:rustdoc.rs-0576 */     {
/* FP:rustdoc.rs-0577 */         // Single fragment with string that contains same bytes as doc.
/* FP:rustdoc.rs-0578 */         return Some((
/* FP:rustdoc.rs-0579 */             Span::new(
/* FP:rustdoc.rs-0580 */                 fragment.span.lo() + crate::rustc_span::BytePos(md_range_lo),
/* FP:rustdoc.rs-0581 */                 fragment.span.lo() + crate::rustc_span::BytePos(md_range_hi),
/* FP:rustdoc.rs-0582 */                 fragment.span.ctxt(),
/* FP:rustdoc.rs-0583 */                 fragment.span.parent(),
/* FP:rustdoc.rs-0584 */             ),
/* FP:rustdoc.rs-0585 */             fragment.from_expansion,
/* FP:rustdoc.rs-0586 */         ));
/* FP:rustdoc.rs-0587 */     }
/* FP:rustdoc.rs-0588 */ 
/* FP:rustdoc.rs-0589 */     let is_all_sugared_doc = fragments.iter().all(|frag| frag.kind == DocFragmentKind::SugaredDoc);
/* FP:rustdoc.rs-0590 */ 
/* FP:rustdoc.rs-0591 */     if !is_all_sugared_doc {
/* FP:rustdoc.rs-0592 */         // This case ignores the markdown outside of the range so that it can
/* FP:rustdoc.rs-0593 */         // work in cases where the markdown is made from several different
/* FP:rustdoc.rs-0594 */         // doc fragments, but the target range does not span across multiple
/* FP:rustdoc.rs-0595 */         // fragments.
/* FP:rustdoc.rs-0596 */         let mut match_data = None;
/* FP:rustdoc.rs-0597 */         let pat = &markdown[md_range.clone()];
/* FP:rustdoc.rs-0598 */         // This heirustic doesn't make sense with a zero-sized range.
/* FP:rustdoc.rs-0599 */         if pat.is_empty() {
/* FP:rustdoc.rs-0600 */             return None;
/* FP:rustdoc.rs-0601 */         }
/* FP:rustdoc.rs-0602 */         for (i, fragment) in fragments.iter().enumerate() {
/* FP:rustdoc.rs-0603 */             if let Ok(snippet) = map.span_to_snippet(fragment.span)
/* FP:rustdoc.rs-0604 */                 && let Some(match_start) = snippet.find(pat)
/* FP:rustdoc.rs-0605 */             {
/* FP:rustdoc.rs-0606 */                 // If there is either a match in a previous fragment, or
/* FP:rustdoc.rs-0607 */                 // multiple matches in this fragment, there is ambiguity.
/* FP:rustdoc.rs-0608 */                 // the snippet cannot be zero-sized, because it matches
/* FP:rustdoc.rs-0609 */                 // the pattern, which is checked to not be zero sized.
/* FP:rustdoc.rs-0610 */                 if match_data.is_none()
/* FP:rustdoc.rs-0611 */                     && !snippet.as_bytes()[match_start + 1..]
/* FP:rustdoc.rs-0612 */                         .windows(pat.len())
/* FP:rustdoc.rs-0613 */                         .any(|s| s == pat.as_bytes())
/* FP:rustdoc.rs-0614 */                 {
/* FP:rustdoc.rs-0615 */                     match_data = Some((i, match_start));
/* FP:rustdoc.rs-0616 */                 } else {
/* FP:rustdoc.rs-0617 */                     // Heuristic produced ambiguity, return nothing.
/* FP:rustdoc.rs-0618 */                     return None;
/* FP:rustdoc.rs-0619 */                 }
/* FP:rustdoc.rs-0620 */             }
/* FP:rustdoc.rs-0621 */         }
/* FP:rustdoc.rs-0622 */         if let Some((i, match_start)) = match_data {
/* FP:rustdoc.rs-0623 */             let fragment = &fragments[i];
/* FP:rustdoc.rs-0624 */             let sp = fragment.span;
/* FP:rustdoc.rs-0625 */             // we need to calculate the span start,
/* FP:rustdoc.rs-0626 */             // then use that in our calculations for the span end
/* FP:rustdoc.rs-0627 */             let lo = sp.lo() + BytePos(match_start as u32);
/* FP:rustdoc.rs-0628 */             return Some((
/* FP:rustdoc.rs-0629 */                 sp.with_lo(lo).with_hi(lo + BytePos((md_range.end - md_range.start) as u32)),
/* FP:rustdoc.rs-0630 */                 fragment.from_expansion,
/* FP:rustdoc.rs-0631 */             ));
/* FP:rustdoc.rs-0632 */         }
/* FP:rustdoc.rs-0633 */         return None;
/* FP:rustdoc.rs-0634 */     }
/* FP:rustdoc.rs-0635 */ 
/* FP:rustdoc.rs-0636 */     let snippet = map.span_to_snippet(span_of_fragments(fragments)?).ok()?;
/* FP:rustdoc.rs-0637 */ 
/* FP:rustdoc.rs-0638 */     let starting_line = markdown[..md_range.start].matches('\n').count();
/* FP:rustdoc.rs-0639 */     let ending_line = starting_line + markdown[md_range.start..md_range.end].matches('\n').count();
/* FP:rustdoc.rs-0640 */ 
/* FP:rustdoc.rs-0641 */     // We use `split_terminator('\n')` instead of `lines()` when counting bytes so that we treat
/* FP:rustdoc.rs-0642 */     // CRLF and LF line endings the same way.
/* FP:rustdoc.rs-0643 */     let mut src_lines = snippet.split_terminator('\n');
/* FP:rustdoc.rs-0644 */     let md_lines = markdown.split_terminator('\n');
/* FP:rustdoc.rs-0645 */ 
/* FP:rustdoc.rs-0646 */     // The number of bytes from the source span to the markdown span that are not part
/* FP:rustdoc.rs-0647 */     // of the markdown, like comment markers.
/* FP:rustdoc.rs-0648 */     let mut start_bytes = 0;
/* FP:rustdoc.rs-0649 */     let mut end_bytes = 0;
/* FP:rustdoc.rs-0650 */ 
/* FP:rustdoc.rs-0651 */     'outer: for (line_no, md_line) in md_lines.enumerate() {
/* FP:rustdoc.rs-0652 */         loop {
/* FP:rustdoc.rs-0653 */             let source_line = src_lines.next()?;
/* FP:rustdoc.rs-0654 */             match source_line.find(md_line) {
/* FP:rustdoc.rs-0655 */                 Some(offset) => {
/* FP:rustdoc.rs-0656 */                     if line_no == starting_line {
/* FP:rustdoc.rs-0657 */                         start_bytes += offset;
/* FP:rustdoc.rs-0658 */ 
/* FP:rustdoc.rs-0659 */                         if starting_line == ending_line {
/* FP:rustdoc.rs-0660 */                             break 'outer;
/* FP:rustdoc.rs-0661 */                         }
/* FP:rustdoc.rs-0662 */                     } else if line_no == ending_line {
/* FP:rustdoc.rs-0663 */                         end_bytes += offset;
/* FP:rustdoc.rs-0664 */                         break 'outer;
/* FP:rustdoc.rs-0665 */                     } else if line_no < starting_line {
/* FP:rustdoc.rs-0666 */                         start_bytes += source_line.len() - md_line.len();
/* FP:rustdoc.rs-0667 */                     } else {
/* FP:rustdoc.rs-0668 */                         end_bytes += source_line.len() - md_line.len();
/* FP:rustdoc.rs-0669 */                     }
/* FP:rustdoc.rs-0670 */                     break;
/* FP:rustdoc.rs-0671 */                 }
/* FP:rustdoc.rs-0672 */                 None => {
/* FP:rustdoc.rs-0673 */                     // Since this is a source line that doesn't include a markdown line,
/* FP:rustdoc.rs-0674 */                     // we have to count the newline that we split from earlier.
/* FP:rustdoc.rs-0675 */                     if line_no <= starting_line {
/* FP:rustdoc.rs-0676 */                         start_bytes += source_line.len() + 1;
/* FP:rustdoc.rs-0677 */                     } else {
/* FP:rustdoc.rs-0678 */                         end_bytes += source_line.len() + 1;
/* FP:rustdoc.rs-0679 */                     }
/* FP:rustdoc.rs-0680 */                 }
/* FP:rustdoc.rs-0681 */             }
/* FP:rustdoc.rs-0682 */         }
/* FP:rustdoc.rs-0683 */     }
/* FP:rustdoc.rs-0684 */ 
/* FP:rustdoc.rs-0685 */     let span = span_of_fragments(fragments)?;
/* FP:rustdoc.rs-0686 */     let src_span = span.from_inner(InnerSpan::new(
/* FP:rustdoc.rs-0687 */         md_range.start + start_bytes,
/* FP:rustdoc.rs-0688 */         md_range.end + start_bytes + end_bytes,
/* FP:rustdoc.rs-0689 */     ));
/* FP:rustdoc.rs-0690 */     Some((
/* FP:rustdoc.rs-0691 */         src_span,
/* FP:rustdoc.rs-0692 */         fragments.iter().any(|frag| frag.span.overlaps(src_span) && frag.from_expansion),
/* FP:rustdoc.rs-0693 */     ))
/* FP:rustdoc.rs-0694 */ }