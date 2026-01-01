/* FP:lib.rs-0001 */ // Diagnostics creation and emission for `rustc`.
/* FP:lib.rs-0002 */ //
/* FP:lib.rs-0003 */ // This module contains the code for creating and emitting diagnostics.
/* FP:lib.rs-0004 */ 
/* FP:lib.rs-0005 */ // tidy-alphabetical-start
/* FP:lib.rs-0006 */ #[allow(internal_features)]
/* FP:lib.rs-0007 */ #[allow(rustc::diagnostic_outside_of_impl)]
/* FP:lib.rs-0008 */ #[allow(rustc::direct_use_of_rustc_type_ir)]
/* FP:lib.rs-0009 */ #[allow(rustc::untranslatable_diagnostic)]
/* FP:lib.rs-0010 */ #[doc(html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")]
/* FP:lib.rs-0011 */ #[doc(rust_logo)]
/* FP:lib.rs-0012 */ #[feature(array_windows)]
/* FP:lib.rs-0013 */ #[feature(assert_matches)]
/* FP:lib.rs-0014 */ #[feature(associated_type_defaults)]
/* FP:lib.rs-0015 */ #[feature(box_patterns)]
/* FP:lib.rs-0016 */ #[feature(default_field_values)]
/* FP:lib.rs-0017 */ #[feature(error_reporter)]
/* FP:lib.rs-0018 */ #[feature(negative_impls)]
/* FP:lib.rs-0019 */ #[feature(never_type)]
/* FP:lib.rs-0020 */ #[feature(rustc_attrs)]
/* FP:lib.rs-0021 */ #[feature(rustdoc_internals)]
/* FP:lib.rs-0022 */ #[feature(try_blocks)]
/* FP:lib.rs-0023 */ #[feature(yeet_expr)]
/* FP:lib.rs-0024 */ // tidy-alphabetical-end
/* FP:lib.rs-0025 */ 
/* FP:lib.rs-0026 */ 
/* FP:lib.rs-0027 */ use std::assert_matches::assert_matches;
/* FP:lib.rs-0028 */ use std::backtrace::{Backtrace, BacktraceStatus};
/* FP:lib.rs-0029 */ use std::borrow::Cow;
/* FP:lib.rs-0030 */ use std::cell::Cell;
/* FP:lib.rs-0031 */ use std::error::Report;
/* FP:lib.rs-0032 */ use std::ffi::OsStr;
/* FP:lib.rs-0033 */ use std::hash::Hash;
/* FP:lib.rs-0034 */ use std::io::Write;
/* FP:lib.rs-0035 */ use std::num::NonZero;
/* FP:lib.rs-0036 */ use std::ops::DerefMut;
/* FP:lib.rs-0037 */ use std::path::{Path, PathBuf};
/* FP:lib.rs-0038 */ use std::{fmt, panic};
/* FP:lib.rs-0039 */ 
/* FP:lib.rs-0040 */ use Level::*;
/* FP:lib.rs-0041 */ pub use codes::*;
/* FP:lib.rs-0042 */ pub use decorate_diag::{BufferedEarlyLint, DecorateDiagCompat, LintBuffer};
/* FP:lib.rs-0043 */ pub use diagnostic::{
/* FP:lib.rs-0044 */     BugAbort, Diag, DiagArgMap, DiagInner, DiagStyledString, Diagnostic, EmissionGuarantee,
/* FP:lib.rs-0045 */     FatalAbort, LintDiagnostic, LintDiagnosticBox, StringPart, Subdiag, Subdiagnostic,
/* FP:lib.rs-0046 */ };
/* FP:lib.rs-0047 */ pub use diagnostic_impls::{
/* FP:lib.rs-0048 */     DiagSymbolList, ElidedLifetimeInPathSubdiag, ExpectedLifetimeParameter,
/* FP:lib.rs-0049 */     IndicateAnonymousLifetime, SingleLabelManySpans,
/* FP:lib.rs-0050 */ };
/* FP:lib.rs-0051 */ pub use emitter::ColorConfig;
/* FP:lib.rs-0052 */ use emitter::{ConfusionType, DynEmitter, Emitter, detect_confusion_type, is_different};
/* FP:lib.rs-0053 */ use crate::rustc_data_structures::AtomicRef;
/* FP:lib.rs-0054 */ use crate::rustc_data_structures::fx::{FxHashSet, FxIndexMap, FxIndexSet};
/* FP:lib.rs-0055 */ use crate::rustc_data_structures::stable_hasher::StableHasher;
/* FP:lib.rs-0056 */ use crate::rustc_data_structures::sync::{DynSend, Lock};
/* FP:lib.rs-0057 */ pub use crate::rustc_error_messages::{
/* FP:lib.rs-0058 */     DiagArg, DiagArgFromDisplay, DiagArgName, DiagArgValue, DiagMessage, FluentBundle, IntoDiagArg,
/* FP:lib.rs-0059 */     LanguageIdentifier, LazyFallbackBundle, MultiSpan, SpanLabel, SubdiagMessage,
/* FP:lib.rs-0060 */     fallback_fluent_bundle, fluent_bundle, into_diag_arg_using_display,
/* FP:lib.rs-0061 */ };
/* FP:lib.rs-0062 */ use rustc_hashes::Hash128;
/* FP:lib.rs-0063 */ pub use crate::rustc_lint_defs::{Applicability, listify, pluralize};
/* FP:lib.rs-0064 */ use crate::rustc_lint_defs::{Lint, LintExpectationId};
/* FP:lib.rs-0065 */ use rustc_macros::{Decodable, Encodable};
/* FP:lib.rs-0066 */ pub use crate::rustc_complete::ErrorGuaranteed;
/* FP:lib.rs-0067 */ pub use crate::rustc_complete::fatal_error::{FatalError, FatalErrorMarker};
/* FP:lib.rs-0068 */ use crate::rustc_complete::source_map::SourceMap;
/* FP:lib.rs-0069 */ use crate::rustc_complete::{BytePos, DUMMY_SP, Loc, Span};
/* FP:lib.rs-0070 */ pub use snippet::Style;
/* FP:lib.rs-0071 */ // Used by external projects such as `rust-gpu`.
/* FP:lib.rs-0072 */ // See https://github.com/rust-lang/rust/pull/115393.
/* FP:lib.rs-0073 */ pub use termcolor::{Color, ColorSpec, WriteColor};
/* FP:lib.rs-0074 */ use tracing::debug;
/* FP:lib.rs-0075 */ 
/* FP:lib.rs-0076 */ use crate::emitter::TimingEvent;
/* FP:lib.rs-0077 */ use crate::registry::Registry;
/* FP:lib.rs-0078 */ use crate::timings::TimingRecord;
/* FP:lib.rs-0079 */ 
/* FP:lib.rs-0093 */ #[cfg(test)]
/* FP:lib.rs-0097 */ 
/* FP:lib.rs-0098 */ pub type PResult<'a, T> = Result<T, Diag<'a>>;
/* FP:lib.rs-0099 */ 
/* FP:lib.rs-0100 */ rustc_fluent_macro::fluent_messages! { "../messages.ftl" }
/* FP:lib.rs-0101 */ 
/* FP:lib.rs-0102 */ // `PResult` is used a lot. Make sure it doesn't unintentionally get bigger.
/* FP:lib.rs-0103 */ #[cfg(target_pointer_width = "64")]
/* FP:lib.rs-0104 */ crate::rustc_data_structures::static_assert_size!(PResult<'_, ()>, 24);
/* FP:lib.rs-0105 */ #[cfg(target_pointer_width = "64")]
/* FP:lib.rs-0106 */ crate::rustc_data_structures::static_assert_size!(PResult<'_, bool>, 24);
/* FP:lib.rs-0107 */ 
/* FP:lib.rs-0108 */ /// Used to avoid depending on `rustc_middle` in `rustc_attr_parsing`.
/* FP:lib.rs-0109 */ /// Always the `TyCtxt`.
/* FP:lib.rs-0110 */ pub trait LintEmitter: Copy {
/* FP:lib.rs-0111 */     type Id: Copy;
/* FP:lib.rs-0112 */     #[track_caller]
/* FP:lib.rs-0113 */     fn emit_node_span_lint(
/* FP:lib.rs-0114 */         self,
/* FP:lib.rs-0115 */         lint: &'static Lint,
/* FP:lib.rs-0116 */         hir_id: Self::Id,
/* FP:lib.rs-0117 */         span: impl Into<MultiSpan>,
/* FP:lib.rs-0118 */         decorator: impl for<'a> LintDiagnostic<'a, ()> + DynSend + 'static,
/* FP:lib.rs-0119 */     );
/* FP:lib.rs-0120 */ }
/* FP:lib.rs-0121 */ 
/* FP:lib.rs-0122 */ #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Encodable, Decodable)]
/* FP:lib.rs-0123 */ pub enum SuggestionStyle {
/* FP:lib.rs-0124 */     /// Hide the suggested code when displaying this suggestion inline.
/* FP:lib.rs-0125 */     HideCodeInline,
/* FP:lib.rs-0126 */     /// Always hide the suggested code but display the message.
/* FP:lib.rs-0127 */     HideCodeAlways,
/* FP:lib.rs-0128 */     /// Do not display this suggestion in the cli output, it is only meant for tools.
/* FP:lib.rs-0129 */     CompletelyHidden,
/* FP:lib.rs-0130 */     /// Always show the suggested code.
/* FP:lib.rs-0131 */     /// This will *not* show the code if the suggestion is inline *and* the suggested code is
/* FP:lib.rs-0132 */     /// empty.
/* FP:lib.rs-0133 */     ShowCode,
/* FP:lib.rs-0134 */     /// Always show the suggested code independently.
/* FP:lib.rs-0135 */     ShowAlways,
/* FP:lib.rs-0136 */ }
/* FP:lib.rs-0137 */ 
/* FP:lib.rs-0138 */ impl SuggestionStyle {
/* FP:lib.rs-0139 */     fn hide_inline(&self) -> bool {
/* FP:lib.rs-0140 */         !matches!(*self, SuggestionStyle::ShowCode)
/* FP:lib.rs-0141 */     }
/* FP:lib.rs-0142 */ }
/* FP:lib.rs-0143 */ 
/* FP:lib.rs-0144 */ /// Represents the help messages seen on a diagnostic.
/* FP:lib.rs-0145 */ #[derive(Clone, Debug, PartialEq, Hash, Encodable, Decodable)]
/* FP:lib.rs-0146 */ pub enum Suggestions {
/* FP:lib.rs-0147 */     /// Indicates that new suggestions can be added or removed from this diagnostic.
/* FP:lib.rs-0148 */     ///
/* FP:lib.rs-0149 */     /// `DiagInner`'s new_* methods initialize the `suggestions` field with
/* FP:lib.rs-0150 */     /// this variant. Also, this is the default variant for `Suggestions`.
/* FP:lib.rs-0151 */     Enabled(Vec<CodeSuggestion>),
/* FP:lib.rs-0152 */     /// Indicates that suggestions cannot be added or removed from this diagnostic.
/* FP:lib.rs-0153 */     ///
/* FP:lib.rs-0154 */     /// Gets toggled when `.seal_suggestions()` is called on the `DiagInner`.
/* FP:lib.rs-0155 */     Sealed(Box<[CodeSuggestion]>),
/* FP:lib.rs-0156 */     /// Indicates that no suggestion is available for this diagnostic.
/* FP:lib.rs-0157 */     ///
/* FP:lib.rs-0158 */     /// Gets toggled when `.disable_suggestions()` is called on the `DiagInner`.
/* FP:lib.rs-0159 */     Disabled,
/* FP:lib.rs-0160 */ }
/* FP:lib.rs-0161 */ 
/* FP:lib.rs-0162 */ impl Suggestions {
/* FP:lib.rs-0163 */     /// Returns the underlying list of suggestions.
/* FP:lib.rs-0164 */     pub fn unwrap_tag(self) -> Vec<CodeSuggestion> {
/* FP:lib.rs-0165 */         match self {
/* FP:lib.rs-0166 */             Suggestions::Enabled(suggestions) => suggestions,
/* FP:lib.rs-0167 */             Suggestions::Sealed(suggestions) => suggestions.into_vec(),
/* FP:lib.rs-0168 */             Suggestions::Disabled => Vec::new(),
/* FP:lib.rs-0169 */         }
/* FP:lib.rs-0170 */     }
/* FP:lib.rs-0171 */ }
/* FP:lib.rs-0172 */ 
/* FP:lib.rs-0173 */ impl Default for Suggestions {
/* FP:lib.rs-0174 */     fn default() -> Self {
/* FP:lib.rs-0175 */         Self::Enabled(vec![])
/* FP:lib.rs-0176 */     }
/* FP:lib.rs-0177 */ }
/* FP:lib.rs-0178 */ 
/* FP:lib.rs-0179 */ #[derive(Clone, Debug, PartialEq, Hash, Encodable, Decodable)]
/* FP:lib.rs-0180 */ pub struct CodeSuggestion {
/* FP:lib.rs-0181 */     /// Each substitute can have multiple variants due to multiple
/* FP:lib.rs-0182 */     /// applicable suggestions
/* FP:lib.rs-0183 */     ///
/* FP:lib.rs-0184 */     /// `foo.bar` might be replaced with `a.b` or `x.y` by replacing
/* FP:lib.rs-0185 */     /// `foo` and `bar` on their own:
/* FP:lib.rs-0186 */     ///
/* FP:lib.rs-0187 */     /// ```ignore (illustrative)
/* FP:lib.rs-0188 */     /// vec![
/* FP:lib.rs-0189 */     ///     Substitution { parts: vec![(0..3, "a"), (4..7, "b")] },
/* FP:lib.rs-0190 */     ///     Substitution { parts: vec![(0..3, "x"), (4..7, "y")] },
/* FP:lib.rs-0191 */     /// ]
/* FP:lib.rs-0192 */     /// ```
/* FP:lib.rs-0193 */     ///
/* FP:lib.rs-0194 */     /// or by replacing the entire span:
/* FP:lib.rs-0195 */     ///
/* FP:lib.rs-0196 */     /// ```ignore (illustrative)
/* FP:lib.rs-0197 */     /// vec![
/* FP:lib.rs-0198 */     ///     Substitution { parts: vec![(0..7, "a.b")] },
/* FP:lib.rs-0199 */     ///     Substitution { parts: vec![(0..7, "x.y")] },
/* FP:lib.rs-0200 */     /// ]
/* FP:lib.rs-0201 */     /// ```
/* FP:lib.rs-0202 */     pub substitutions: Vec<Substitution>,
/* FP:lib.rs-0203 */     pub msg: DiagMessage,
/* FP:lib.rs-0204 */     /// Visual representation of this suggestion.
/* FP:lib.rs-0205 */     pub style: SuggestionStyle,
/* FP:lib.rs-0206 */     /// Whether or not the suggestion is approximate
/* FP:lib.rs-0207 */     ///
/* FP:lib.rs-0208 */     /// Sometimes we may show suggestions with placeholders,
/* FP:lib.rs-0209 */     /// which are useful for users but not useful for
/* FP:lib.rs-0210 */     /// tools like rustfix
/* FP:lib.rs-0211 */     pub applicability: Applicability,
/* FP:lib.rs-0212 */ }
/* FP:lib.rs-0213 */ 
/* FP:lib.rs-0214 */ #[derive(Clone, Debug, PartialEq, Hash, Encodable, Decodable)]
/* FP:lib.rs-0215 */ /// See the docs on `CodeSuggestion::substitutions`
/* FP:lib.rs-0216 */ pub struct Substitution {
/* FP:lib.rs-0217 */     pub parts: Vec<SubstitutionPart>,
/* FP:lib.rs-0218 */ }
/* FP:lib.rs-0219 */ 
/* FP:lib.rs-0220 */ #[derive(Clone, Debug, PartialEq, Hash, Encodable, Decodable)]
/* FP:lib.rs-0221 */ pub struct SubstitutionPart {
/* FP:lib.rs-0222 */     pub span: Span,
/* FP:lib.rs-0223 */     pub snippet: String,
/* FP:lib.rs-0224 */ }
/* FP:lib.rs-0225 */ 
/* FP:lib.rs-0226 */ /// Used to translate between `Span`s and byte positions within a single output line in highlighted
/* FP:lib.rs-0227 */ /// code of structured suggestions.
/* FP:lib.rs-0228 */ #[derive(Debug, Clone, Copy)]
/* FP:lib.rs-0229 */ pub(crate) struct SubstitutionHighlight {
/* FP:lib.rs-0230 */     start: usize,
/* FP:lib.rs-0231 */     end: usize,
/* FP:lib.rs-0232 */ }
/* FP:lib.rs-0233 */ 
/* FP:lib.rs-0234 */ impl SubstitutionPart {
/* FP:lib.rs-0235 */     pub fn is_addition(&self, sm: &SourceMap) -> bool {
/* FP:lib.rs-0236 */         !self.snippet.is_empty() && !self.replaces_meaningful_content(sm)
/* FP:lib.rs-0237 */     }
/* FP:lib.rs-0238 */ 
/* FP:lib.rs-0239 */     pub fn is_deletion(&self, sm: &SourceMap) -> bool {
/* FP:lib.rs-0240 */         self.snippet.trim().is_empty() && self.replaces_meaningful_content(sm)
/* FP:lib.rs-0241 */     }
/* FP:lib.rs-0242 */ 
/* FP:lib.rs-0243 */     pub fn is_replacement(&self, sm: &SourceMap) -> bool {
/* FP:lib.rs-0244 */         !self.snippet.is_empty() && self.replaces_meaningful_content(sm)
/* FP:lib.rs-0245 */     }
/* FP:lib.rs-0246 */ 
/* FP:lib.rs-0247 */     /// Whether this is a replacement that overwrites source with a snippet
/* FP:lib.rs-0248 */     /// in a way that isn't a superset of the original string. For example,
/* FP:lib.rs-0249 */     /// replacing "abc" with "abcde" is not destructive, but replacing it
/* FP:lib.rs-0250 */     /// it with "abx" is, since the "c" character is lost.
/* FP:lib.rs-0251 */     pub fn is_destructive_replacement(&self, sm: &SourceMap) -> bool {
/* FP:lib.rs-0252 */         self.is_replacement(sm)
/* FP:lib.rs-0253 */             && !sm
/* FP:lib.rs-0254 */                 .span_to_snippet(self.span)
/* FP:lib.rs-0255 */                 .is_ok_and(|snippet| as_substr(snippet.trim(), self.snippet.trim()).is_some())
/* FP:lib.rs-0256 */     }
/* FP:lib.rs-0257 */ 
/* FP:lib.rs-0258 */     fn replaces_meaningful_content(&self, sm: &SourceMap) -> bool {
/* FP:lib.rs-0259 */         sm.span_to_snippet(self.span)
/* FP:lib.rs-0260 */             .map_or(!self.span.is_empty(), |snippet| !snippet.trim().is_empty())
/* FP:lib.rs-0261 */     }
/* FP:lib.rs-0262 */ 
/* FP:lib.rs-0263 */     /// Try to turn a replacement into an addition when the span that is being
/* FP:lib.rs-0264 */     /// overwritten matches either the prefix or suffix of the replacement.
/* FP:lib.rs-0265 */     fn trim_trivial_replacements(&mut self, sm: &SourceMap) {
/* FP:lib.rs-0266 */         if self.snippet.is_empty() {
/* FP:lib.rs-0267 */             return;
/* FP:lib.rs-0268 */         }
/* FP:lib.rs-0269 */         let Ok(snippet) = sm.span_to_snippet(self.span) else {
/* FP:lib.rs-0270 */             return;
/* FP:lib.rs-0271 */         };
/* FP:lib.rs-0272 */ 
/* FP:lib.rs-0273 */         if let Some((prefix, substr, suffix)) = as_substr(&snippet, &self.snippet) {
/* FP:lib.rs-0274 */             self.span = Span::new(
/* FP:lib.rs-0275 */                 self.span.lo() + BytePos(prefix as u32),
/* FP:lib.rs-0276 */                 self.span.hi() - BytePos(suffix as u32),
/* FP:lib.rs-0277 */                 self.span.ctxt(),
/* FP:lib.rs-0278 */                 self.span.parent(),
/* FP:lib.rs-0279 */             );
/* FP:lib.rs-0280 */             self.snippet = substr.to_string();
/* FP:lib.rs-0281 */         }
/* FP:lib.rs-0282 */     }
/* FP:lib.rs-0283 */ }
/* FP:lib.rs-0284 */ 
/* FP:lib.rs-0285 */ /// Given an original string like `AACC`, and a suggestion like `AABBCC`, try to detect
/* FP:lib.rs-0286 */ /// the case where a substring of the suggestion is "sandwiched" in the original, like
/* FP:lib.rs-0287 */ /// `BB` is. Return the length of the prefix, the "trimmed" suggestion, and the length
/* FP:lib.rs-0288 */ /// of the suffix.
/* FP:lib.rs-0289 */ fn as_substr<'a>(original: &'a str, suggestion: &'a str) -> Option<(usize, &'a str, usize)> {
/* FP:lib.rs-0290 */     let common_prefix = original
/* FP:lib.rs-0291 */         .chars()
/* FP:lib.rs-0292 */         .zip(suggestion.chars())
/* FP:lib.rs-0293 */         .take_while(|(c1, c2)| c1 == c2)
/* FP:lib.rs-0294 */         .map(|(c, _)| c.len_utf8())
/* FP:lib.rs-0295 */         .sum();
/* FP:lib.rs-0296 */     let original = &original[common_prefix..];
/* FP:lib.rs-0297 */     let suggestion = &suggestion[common_prefix..];
/* FP:lib.rs-0298 */     if suggestion.ends_with(original) {
/* FP:lib.rs-0299 */         let common_suffix = original.len();
/* FP:lib.rs-0300 */         Some((common_prefix, &suggestion[..suggestion.len() - original.len()], common_suffix))
/* FP:lib.rs-0301 */     } else {
/* FP:lib.rs-0302 */         None
/* FP:lib.rs-0303 */     }
/* FP:lib.rs-0304 */ }
/* FP:lib.rs-0305 */ 
/* FP:lib.rs-0306 */ impl CodeSuggestion {
/* FP:lib.rs-0307 */     /// Returns the assembled code suggestions, whether they should be shown with an underline
/* FP:lib.rs-0308 */     /// and whether the substitution only differs in capitalization.
/* FP:lib.rs-0309 */     pub(crate) fn splice_lines(
/* FP:lib.rs-0310 */         &self,
/* FP:lib.rs-0311 */         sm: &SourceMap,
/* FP:lib.rs-0312 */     ) -> Vec<(String, Vec<SubstitutionPart>, Vec<Vec<SubstitutionHighlight>>, ConfusionType)> {
/* FP:lib.rs-0313 */         // For the `Vec<Vec<SubstitutionHighlight>>` value, the first level of the vector
/* FP:lib.rs-0314 */         // corresponds to the output snippet's lines, while the second level corresponds to the
/* FP:lib.rs-0315 */         // substrings within that line that should be highlighted.
/* FP:lib.rs-0316 */ 
/* FP:lib.rs-0317 */         use crate::rustc_complete::{CharPos, Pos};
/* FP:lib.rs-0318 */ 
/* FP:lib.rs-0319 */         /// Extracts a substring from the provided `line_opt` based on the specified low and high
/* FP:lib.rs-0320 */         /// indices, appends it to the given buffer `buf`, and returns the count of newline
/* FP:lib.rs-0321 */         /// characters in the substring for accurate highlighting. If `line_opt` is `None`, a
/* FP:lib.rs-0322 */         /// newline character is appended to the buffer, and 0 is returned.
/* FP:lib.rs-0323 */         ///
/* FP:lib.rs-0324 */         /// ## Returns
/* FP:lib.rs-0325 */         ///
/* FP:lib.rs-0326 */         /// The count of newline characters in the extracted substring.
/* FP:lib.rs-0327 */         fn push_trailing(
/* FP:lib.rs-0328 */             buf: &mut String,
/* FP:lib.rs-0329 */             line_opt: Option<&Cow<'_, str>>,
/* FP:lib.rs-0330 */             lo: &Loc,
/* FP:lib.rs-0331 */             hi_opt: Option<&Loc>,
/* FP:lib.rs-0332 */         ) -> usize {
/* FP:lib.rs-0333 */             let mut line_count = 0;
/* FP:lib.rs-0334 */             // Convert CharPos to Usize, as CharPose is character offset
/* FP:lib.rs-0335 */             // Extract low index and high index
/* FP:lib.rs-0336 */             let (lo, hi_opt) = (lo.col.to_usize(), hi_opt.map(|hi| hi.col.to_usize()));
/* FP:lib.rs-0337 */             if let Some(line) = line_opt {
/* FP:lib.rs-0338 */                 if let Some(lo) = line.char_indices().map(|(i, _)| i).nth(lo) {
/* FP:lib.rs-0339 */                     // Get high index while account for rare unicode and emoji with char_indices
/* FP:lib.rs-0340 */                     let hi_opt = hi_opt.and_then(|hi| line.char_indices().map(|(i, _)| i).nth(hi));
/* FP:lib.rs-0341 */                     match hi_opt {
/* FP:lib.rs-0342 */                         // If high index exist, take string from low to high index
/* FP:lib.rs-0343 */                         Some(hi) if hi > lo => {
/* FP:lib.rs-0344 */                             // count how many '\n' exist
/* FP:lib.rs-0345 */                             line_count = line[lo..hi].matches('\n').count();
/* FP:lib.rs-0346 */                             buf.push_str(&line[lo..hi])
/* FP:lib.rs-0347 */                         }
/* FP:lib.rs-0348 */                         Some(_) => (),
/* FP:lib.rs-0349 */                         // If high index absence, take string from low index till end string.len
/* FP:lib.rs-0350 */                         None => {
/* FP:lib.rs-0351 */                             // count how many '\n' exist
/* FP:lib.rs-0352 */                             line_count = line[lo..].matches('\n').count();
/* FP:lib.rs-0353 */                             buf.push_str(&line[lo..])
/* FP:lib.rs-0354 */                         }
/* FP:lib.rs-0355 */                     }
/* FP:lib.rs-0356 */                 }
/* FP:lib.rs-0357 */                 // If high index is None
/* FP:lib.rs-0358 */                 if hi_opt.is_none() {
/* FP:lib.rs-0359 */                     buf.push('\n');
/* FP:lib.rs-0360 */                 }
/* FP:lib.rs-0361 */             }
/* FP:lib.rs-0362 */             line_count
/* FP:lib.rs-0363 */         }
/* FP:lib.rs-0364 */ 
/* FP:lib.rs-0365 */         assert!(!self.substitutions.is_empty());
/* FP:lib.rs-0366 */ 
/* FP:lib.rs-0367 */         self.substitutions
/* FP:lib.rs-0368 */             .iter()
/* FP:lib.rs-0369 */             .filter(|subst| {
/* FP:lib.rs-0370 */                 // Suggestions coming from macros can have malformed spans. This is a heavy
/* FP:lib.rs-0371 */                 // handed approach to avoid ICEs by ignoring the suggestion outright.
/* FP:lib.rs-0372 */                 let invalid = subst.parts.iter().any(|item| sm.is_valid_span(item.span).is_err());
/* FP:lib.rs-0373 */                 if invalid {
/* FP:lib.rs-0374 */                     debug!("splice_lines: suggestion contains an invalid span: {:?}", subst);
/* FP:lib.rs-0375 */                 }
/* FP:lib.rs-0376 */                 !invalid
/* FP:lib.rs-0377 */             })
/* FP:lib.rs-0378 */             .cloned()
/* FP:lib.rs-0379 */             .filter_map(|mut substitution| {
/* FP:lib.rs-0380 */                 // Assumption: all spans are in the same file, and all spans
/* FP:lib.rs-0381 */                 // are disjoint. Sort in ascending order.
/* FP:lib.rs-0382 */                 substitution.parts.sort_by_key(|part| part.span.lo());
/* FP:lib.rs-0383 */ 
/* FP:lib.rs-0384 */                 // Find the bounding span.
/* FP:lib.rs-0385 */                 let lo = substitution.parts.iter().map(|part| part.span.lo()).min()?;
/* FP:lib.rs-0386 */                 let hi = substitution.parts.iter().map(|part| part.span.hi()).max()?;
/* FP:lib.rs-0387 */                 let bounding_span = Span::with_root_ctxt(lo, hi);
/* FP:lib.rs-0388 */                 // The different spans might belong to different contexts, if so ignore suggestion.
/* FP:lib.rs-0389 */                 let lines = sm.span_to_lines(bounding_span).ok()?;
/* FP:lib.rs-0390 */                 assert!(!lines.lines.is_empty() || bounding_span.is_dummy());
/* FP:lib.rs-0391 */ 
/* FP:lib.rs-0392 */                 // We can't splice anything if the source is unavailable.
/* FP:lib.rs-0393 */                 if !sm.ensure_source_file_source_present(&lines.file) {
/* FP:lib.rs-0394 */                     return None;
/* FP:lib.rs-0395 */                 }
/* FP:lib.rs-0396 */ 
/* FP:lib.rs-0397 */                 let mut highlights = vec![];
/* FP:lib.rs-0398 */                 // To build up the result, we do this for each span:
/* FP:lib.rs-0399 */                 // - push the line segment trailing the previous span
/* FP:lib.rs-0400 */                 //   (at the beginning a "phantom" span pointing at the start of the line)
/* FP:lib.rs-0401 */                 // - push lines between the previous and current span (if any)
/* FP:lib.rs-0402 */                 // - if the previous and current span are not on the same line
/* FP:lib.rs-0403 */                 //   push the line segment leading up to the current span
/* FP:lib.rs-0404 */                 // - splice in the span substitution
/* FP:lib.rs-0405 */                 //
/* FP:lib.rs-0406 */                 // Finally push the trailing line segment of the last span
/* FP:lib.rs-0407 */                 let sf = &lines.file;
/* FP:lib.rs-0408 */                 let mut prev_hi = sm.lookup_char_pos(bounding_span.lo());
/* FP:lib.rs-0409 */                 prev_hi.col = CharPos::from_usize(0);
/* FP:lib.rs-0410 */                 let mut prev_line =
/* FP:lib.rs-0411 */                     lines.lines.get(0).and_then(|line0| sf.get_line(line0.line_index));
/* FP:lib.rs-0412 */                 let mut buf = String::new();
/* FP:lib.rs-0413 */ 
/* FP:lib.rs-0414 */                 let mut line_highlight = vec![];
/* FP:lib.rs-0415 */                 // We need to keep track of the difference between the existing code and the added
/* FP:lib.rs-0416 */                 // or deleted code in order to point at the correct column *after* substitution.
/* FP:lib.rs-0417 */                 let mut acc = 0;
/* FP:lib.rs-0418 */                 let mut confusion_type = ConfusionType::None;
/* FP:lib.rs-0419 */                 for part in &mut substitution.parts {
/* FP:lib.rs-0420 */                     // If this is a replacement of, e.g. `"a"` into `"ab"`, adjust the
/* FP:lib.rs-0421 */                     // suggestion and snippet to look as if we just suggested to add
/* FP:lib.rs-0422 */                     // `"b"`, which is typically much easier for the user to understand.
/* FP:lib.rs-0423 */                     part.trim_trivial_replacements(sm);
/* FP:lib.rs-0424 */ 
/* FP:lib.rs-0425 */                     let part_confusion = detect_confusion_type(sm, &part.snippet, part.span);
/* FP:lib.rs-0426 */                     confusion_type = confusion_type.combine(part_confusion);
/* FP:lib.rs-0427 */                     let cur_lo = sm.lookup_char_pos(part.span.lo());
/* FP:lib.rs-0428 */                     if prev_hi.line == cur_lo.line {
/* FP:lib.rs-0429 */                         let mut count =
/* FP:lib.rs-0430 */                             push_trailing(&mut buf, prev_line.as_ref(), &prev_hi, Some(&cur_lo));
/* FP:lib.rs-0431 */                         while count > 0 {
/* FP:lib.rs-0432 */                             highlights.push(std::mem::take(&mut line_highlight));
/* FP:lib.rs-0433 */                             acc = 0;
/* FP:lib.rs-0434 */                             count -= 1;
/* FP:lib.rs-0435 */                         }
/* FP:lib.rs-0436 */                     } else {
/* FP:lib.rs-0437 */                         acc = 0;
/* FP:lib.rs-0438 */                         highlights.push(std::mem::take(&mut line_highlight));
/* FP:lib.rs-0439 */                         let mut count = push_trailing(&mut buf, prev_line.as_ref(), &prev_hi, None);
/* FP:lib.rs-0440 */                         while count > 0 {
/* FP:lib.rs-0441 */                             highlights.push(std::mem::take(&mut line_highlight));
/* FP:lib.rs-0442 */                             count -= 1;
/* FP:lib.rs-0443 */                         }
/* FP:lib.rs-0444 */                         // push lines between the previous and current span (if any)
/* FP:lib.rs-0445 */                         for idx in prev_hi.line..(cur_lo.line - 1) {
/* FP:lib.rs-0446 */                             if let Some(line) = sf.get_line(idx) {
/* FP:lib.rs-0447 */                                 buf.push_str(line.as_ref());
/* FP:lib.rs-0448 */                                 buf.push('\n');
/* FP:lib.rs-0449 */                                 highlights.push(std::mem::take(&mut line_highlight));
/* FP:lib.rs-0450 */                             }
/* FP:lib.rs-0451 */                         }
/* FP:lib.rs-0452 */                         if let Some(cur_line) = sf.get_line(cur_lo.line - 1) {
/* FP:lib.rs-0453 */                             let end = match cur_line.char_indices().nth(cur_lo.col.to_usize()) {
/* FP:lib.rs-0454 */                                 Some((i, _)) => i,
/* FP:lib.rs-0455 */                                 None => cur_line.len(),
/* FP:lib.rs-0456 */                             };
/* FP:lib.rs-0457 */                             buf.push_str(&cur_line[..end]);
/* FP:lib.rs-0458 */                         }
/* FP:lib.rs-0459 */                     }
/* FP:lib.rs-0460 */                     // Add a whole line highlight per line in the snippet.
/* FP:lib.rs-0461 */                     let len: isize = part
/* FP:lib.rs-0462 */                         .snippet
/* FP:lib.rs-0463 */                         .split('\n')
/* FP:lib.rs-0464 */                         .next()
/* FP:lib.rs-0465 */                         .unwrap_or(&part.snippet)
/* FP:lib.rs-0466 */                         .chars()
/* FP:lib.rs-0467 */                         .map(|c| match c {
/* FP:lib.rs-0468 */                             '\t' => 4,
/* FP:lib.rs-0469 */                             _ => 1,
/* FP:lib.rs-0470 */                         })
/* FP:lib.rs-0471 */                         .sum();
/* FP:lib.rs-0472 */                     if !is_different(sm, &part.snippet, part.span) {
/* FP:lib.rs-0473 */                         // Account for cases where we are suggesting the same code that's already
/* FP:lib.rs-0474 */                         // there. This shouldn't happen often, but in some cases for multipart
/* FP:lib.rs-0475 */                         // suggestions it's much easier to handle it here than in the origin.
/* FP:lib.rs-0476 */                     } else {
/* FP:lib.rs-0477 */                         line_highlight.push(SubstitutionHighlight {
/* FP:lib.rs-0478 */                             start: (cur_lo.col.0 as isize + acc) as usize,
/* FP:lib.rs-0479 */                             end: (cur_lo.col.0 as isize + acc + len) as usize,
/* FP:lib.rs-0480 */                         });
/* FP:lib.rs-0481 */                     }
/* FP:lib.rs-0482 */                     buf.push_str(&part.snippet);
/* FP:lib.rs-0483 */                     let cur_hi = sm.lookup_char_pos(part.span.hi());
/* FP:lib.rs-0484 */                     // Account for the difference between the width of the current code and the
/* FP:lib.rs-0485 */                     // snippet being suggested, so that the *later* suggestions are correctly
/* FP:lib.rs-0486 */                     // aligned on the screen. Note that cur_hi and cur_lo can be on different
/* FP:lib.rs-0487 */                     // lines, so cur_hi.col can be smaller than cur_lo.col
/* FP:lib.rs-0488 */                     acc += len - (cur_hi.col.0 as isize - cur_lo.col.0 as isize);
/* FP:lib.rs-0489 */                     prev_hi = cur_hi;
/* FP:lib.rs-0490 */                     prev_line = sf.get_line(prev_hi.line - 1);
/* FP:lib.rs-0491 */                     for line in part.snippet.split('\n').skip(1) {
/* FP:lib.rs-0492 */                         acc = 0;
/* FP:lib.rs-0493 */                         highlights.push(std::mem::take(&mut line_highlight));
/* FP:lib.rs-0494 */                         let end: usize = line
/* FP:lib.rs-0495 */                             .chars()
/* FP:lib.rs-0496 */                             .map(|c| match c {
/* FP:lib.rs-0497 */                                 '\t' => 4,
/* FP:lib.rs-0498 */                                 _ => 1,
/* FP:lib.rs-0499 */                             })
/* FP:lib.rs-0500 */                             .sum();
/* FP:lib.rs-0501 */                         line_highlight.push(SubstitutionHighlight { start: 0, end });
/* FP:lib.rs-0502 */                     }
/* FP:lib.rs-0503 */                 }
/* FP:lib.rs-0504 */                 highlights.push(std::mem::take(&mut line_highlight));
/* FP:lib.rs-0505 */                 // if the replacement already ends with a newline, don't print the next line
/* FP:lib.rs-0506 */                 if !buf.ends_with('\n') {
/* FP:lib.rs-0507 */                     push_trailing(&mut buf, prev_line.as_ref(), &prev_hi, None);
/* FP:lib.rs-0508 */                 }
/* FP:lib.rs-0509 */                 // remove trailing newlines
/* FP:lib.rs-0510 */                 while buf.ends_with('\n') {
/* FP:lib.rs-0511 */                     buf.pop();
/* FP:lib.rs-0512 */                 }
/* FP:lib.rs-0513 */                 if highlights.iter().all(|parts| parts.is_empty()) {
/* FP:lib.rs-0514 */                     None
/* FP:lib.rs-0515 */                 } else {
/* FP:lib.rs-0516 */                     Some((buf, substitution.parts, highlights, confusion_type))
/* FP:lib.rs-0517 */                 }
/* FP:lib.rs-0518 */             })
/* FP:lib.rs-0519 */             .collect()
/* FP:lib.rs-0520 */     }
/* FP:lib.rs-0521 */ }
/* FP:lib.rs-0522 */ 
/* FP:lib.rs-0523 */ /// Signifies that the compiler died with an explicit call to `.bug`
/* FP:lib.rs-0524 */ /// or `.span_bug` rather than a failed assertion, etc.
/* FP:lib.rs-0525 */ pub struct ExplicitBug;
/* FP:lib.rs-0526 */ 
/* FP:lib.rs-0527 */ /// Signifies that the compiler died due to a delayed bug rather than a failed
/* FP:lib.rs-0528 */ /// assertion, etc.
/* FP:lib.rs-0529 */ pub struct DelayedBugPanic;
/* FP:lib.rs-0530 */ 
/* FP:lib.rs-0531 */ /// A `DiagCtxt` deals with errors and other compiler output.
/* FP:lib.rs-0532 */ /// Certain errors (fatal, bug, unimpl) may cause immediate exit,
/* FP:lib.rs-0533 */ /// others log errors for later reporting.
/* FP:lib.rs-0534 */ pub struct DiagCtxt {
/* FP:lib.rs-0535 */     inner: Lock<DiagCtxtInner>,
/* FP:lib.rs-0536 */ }
/* FP:lib.rs-0537 */ 
/* FP:lib.rs-0538 */ #[derive(Copy, Clone)]
/* FP:lib.rs-0539 */ pub struct DiagCtxtHandle<'a> {
/* FP:lib.rs-0540 */     dcx: &'a DiagCtxt,
/* FP:lib.rs-0541 */     /// Some contexts create `DiagCtxtHandle` with this field set, and thus all
/* FP:lib.rs-0542 */     /// errors emitted with it will automatically taint when emitting errors.
/* FP:lib.rs-0543 */     tainted_with_errors: Option<&'a Cell<Option<ErrorGuaranteed>>>,
/* FP:lib.rs-0544 */ }
/* FP:lib.rs-0545 */ 
/* FP:lib.rs-0546 */ impl<'a> std::ops::Deref for DiagCtxtHandle<'a> {
/* FP:lib.rs-0547 */     type Target = &'a DiagCtxt;
/* FP:lib.rs-0548 */ 
/* FP:lib.rs-0549 */     fn deref(&self) -> &Self::Target {
/* FP:lib.rs-0550 */         &self.dcx
/* FP:lib.rs-0551 */     }
/* FP:lib.rs-0552 */ }
/* FP:lib.rs-0553 */ 
/* FP:lib.rs-0554 */ /// This inner struct exists to keep it all behind a single lock;
/* FP:lib.rs-0555 */ /// this is done to prevent possible deadlocks in a multi-threaded compiler,
/* FP:lib.rs-0556 */ /// as well as inconsistent state observation.
/* FP:lib.rs-0557 */ struct DiagCtxtInner {
/* FP:lib.rs-0558 */     flags: DiagCtxtFlags,
/* FP:lib.rs-0559 */ 
/* FP:lib.rs-0560 */     registry: Registry,
/* FP:lib.rs-0561 */ 
/* FP:lib.rs-0562 */     /// The error guarantees from all emitted errors. The length gives the error count.
/* FP:lib.rs-0563 */     err_guars: Vec<ErrorGuaranteed>,
/* FP:lib.rs-0564 */     /// The error guarantee from all emitted lint errors. The length gives the
/* FP:lib.rs-0565 */     /// lint error count.
/* FP:lib.rs-0566 */     lint_err_guars: Vec<ErrorGuaranteed>,
/* FP:lib.rs-0567 */     /// The delayed bugs and their error guarantees.
/* FP:lib.rs-0568 */     delayed_bugs: Vec<(DelayedDiagInner, ErrorGuaranteed)>,
/* FP:lib.rs-0569 */ 
/* FP:lib.rs-0570 */     /// The error count shown to the user at the end.
/* FP:lib.rs-0571 */     deduplicated_err_count: usize,
/* FP:lib.rs-0572 */     /// The warning count shown to the user at the end.
/* FP:lib.rs-0573 */     deduplicated_warn_count: usize,
/* FP:lib.rs-0574 */ 
/* FP:lib.rs-0575 */     emitter: Box<DynEmitter>,
/* FP:lib.rs-0576 */ 
/* FP:lib.rs-0577 */     /// Must we produce a diagnostic to justify the use of the expensive
/* FP:lib.rs-0578 */     /// `trimmed_def_paths` function? Backtrace is the location of the call.
/* FP:lib.rs-0579 */     must_produce_diag: Option<Backtrace>,
/* FP:lib.rs-0580 */ 
/* FP:lib.rs-0581 */     /// Has this diagnostic context printed any diagnostics? (I.e. has
/* FP:lib.rs-0582 */     /// `self.emitter.emit_diagnostic()` been called?
/* FP:lib.rs-0583 */     has_printed: bool,
/* FP:lib.rs-0584 */ 
/* FP:lib.rs-0585 */     /// This flag indicates that an expected diagnostic was emitted and suppressed.
/* FP:lib.rs-0586 */     /// This is used for the `must_produce_diag` check.
/* FP:lib.rs-0587 */     suppressed_expected_diag: bool,
/* FP:lib.rs-0588 */ 
/* FP:lib.rs-0589 */     /// This set contains the code of all emitted diagnostics to avoid
/* FP:lib.rs-0590 */     /// emitting the same diagnostic with extended help (`--teach`) twice, which
/* FP:lib.rs-0591 */     /// would be unnecessary repetition.
/* FP:lib.rs-0592 */     taught_diagnostics: FxHashSet<ErrCode>,
/* FP:lib.rs-0593 */ 
/* FP:lib.rs-0594 */     /// Used to suggest rustc --explain `<error code>`
/* FP:lib.rs-0595 */     emitted_diagnostic_codes: FxIndexSet<ErrCode>,
/* FP:lib.rs-0596 */ 
/* FP:lib.rs-0597 */     /// This set contains a hash of every diagnostic that has been emitted by
/* FP:lib.rs-0598 */     /// this `DiagCtxt`. These hashes is used to avoid emitting the same error
/* FP:lib.rs-0599 */     /// twice.
/* FP:lib.rs-0600 */     emitted_diagnostics: FxHashSet<Hash128>,
/* FP:lib.rs-0601 */ 
/* FP:lib.rs-0602 */     /// Stashed diagnostics emitted in one stage of the compiler that may be
/* FP:lib.rs-0603 */     /// stolen and emitted/cancelled by other stages (e.g. to improve them and
/* FP:lib.rs-0604 */     /// add more information). All stashed diagnostics must be emitted with
/* FP:lib.rs-0605 */     /// `emit_stashed_diagnostics` by the time the `DiagCtxtInner` is dropped,
/* FP:lib.rs-0606 */     /// otherwise an assertion failure will occur.
/* FP:lib.rs-0607 */     stashed_diagnostics:
/* FP:lib.rs-0608 */         FxIndexMap<StashKey, FxIndexMap<Span, (DiagInner, Option<ErrorGuaranteed>)>>,
/* FP:lib.rs-0609 */ 
/* FP:lib.rs-0610 */     future_breakage_diagnostics: Vec<DiagInner>,
/* FP:lib.rs-0611 */ 
/* FP:lib.rs-0612 */     /// expected diagnostic will have the level `Expect` which additionally
/* FP:lib.rs-0613 */     /// carries the [`LintExpectationId`] of the expectation that can be
/* FP:lib.rs-0614 */     /// marked as fulfilled. This is a collection of all [`LintExpectationId`]s
/* FP:lib.rs-0615 */     /// that have been marked as fulfilled this way.
/* FP:lib.rs-0616 */     ///
/* FP:lib.rs-0617 */     /// Emitting expectations after having stolen this field can happen. In particular, an
/* FP:lib.rs-0618 */     /// `#[expect(warnings)]` can easily make the `UNFULFILLED_LINT_EXPECTATIONS` lint expect
/* FP:lib.rs-0619 */     /// itself. To avoid needless complexity in this corner case, we tolerate failing to track
/* FP:lib.rs-0620 */     /// those expectations.
/* FP:lib.rs-0621 */     ///
/* FP:lib.rs-0622 */     /// [RFC-2383]: https://rust-lang.github.io/rfcs/2383-lint-reasons.html
/* FP:lib.rs-0623 */     fulfilled_expectations: FxIndexSet<LintExpectationId>,
/* FP:lib.rs-0624 */ 
/* FP:lib.rs-0625 */     /// The file where the ICE information is stored. This allows delayed_span_bug backtraces to be
/* FP:lib.rs-0626 */     /// stored along side the main panic backtrace.
/* FP:lib.rs-0627 */     ice_file: Option<PathBuf>,
/* FP:lib.rs-0628 */ }
/* FP:lib.rs-0629 */ 
/* FP:lib.rs-0630 */ /// A key denoting where from a diagnostic was stashed.
/* FP:lib.rs-0631 */ #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
/* FP:lib.rs-0632 */ pub enum StashKey {
/* FP:lib.rs-0633 */     ItemNoType,
/* FP:lib.rs-0634 */     UnderscoreForArrayLengths,
/* FP:lib.rs-0635 */     EarlySyntaxWarning,
/* FP:lib.rs-0636 */     CallIntoMethod,
/* FP:lib.rs-0637 */     /// When an invalid lifetime e.g. `'2` should be reinterpreted
/* FP:lib.rs-0638 */     /// as a char literal in the parser
/* FP:lib.rs-0639 */     LifetimeIsChar,
/* FP:lib.rs-0640 */     /// Maybe there was a typo where a comma was forgotten before
/* FP:lib.rs-0641 */     /// FRU syntax
/* FP:lib.rs-0642 */     MaybeFruTypo,
/* FP:lib.rs-0643 */     CallAssocMethod,
/* FP:lib.rs-0644 */     AssociatedTypeSuggestion,
/* FP:lib.rs-0645 */     /// Query cycle detected, stashing in favor of a better error.
/* FP:lib.rs-0646 */     Cycle,
/* FP:lib.rs-0647 */     UndeterminedMacroResolution,
/* FP:lib.rs-0648 */     /// Used by `Parser::maybe_recover_trailing_expr`
/* FP:lib.rs-0649 */     ExprInPat,
/* FP:lib.rs-0650 */     /// If in the parser we detect a field expr with turbofish generic params it's possible that
/* FP:lib.rs-0651 */     /// it's a method call without parens. If later on in `hir_typeck` we find out that this is
/* FP:lib.rs-0652 */     /// the case we suppress this message and we give a better suggestion.
/* FP:lib.rs-0653 */     GenericInFieldExpr,
/* FP:lib.rs-0654 */ }
/* FP:lib.rs-0655 */ 
/* FP:lib.rs-0656 */ fn default_track_diagnostic<R>(diag: DiagInner, f: &mut dyn FnMut(DiagInner) -> R) -> R {
/* FP:lib.rs-0657 */     (*f)(diag)
/* FP:lib.rs-0658 */ }
/* FP:lib.rs-0659 */ 
/* FP:lib.rs-0660 */ /// Diagnostics emitted by `DiagCtxtInner::emit_diagnostic` are passed through this function. Used
/* FP:lib.rs-0661 */ /// for tracking by incremental, to replay diagnostics as necessary.
/* FP:lib.rs-0662 */ pub static TRACK_DIAGNOSTIC: AtomicRef<
/* FP:lib.rs-0663 */     fn(DiagInner, &mut dyn FnMut(DiagInner) -> Option<ErrorGuaranteed>) -> Option<ErrorGuaranteed>,
/* FP:lib.rs-0664 */ > = AtomicRef::new(&(default_track_diagnostic as _));
/* FP:lib.rs-0665 */ 
/* FP:lib.rs-0666 */ #[derive(Copy, Clone, Default)]
/* FP:lib.rs-0667 */ pub struct DiagCtxtFlags {
/* FP:lib.rs-0668 */     /// If false, warning-level lints are suppressed.
/* FP:lib.rs-0669 */     /// (rustc: see `--allow warnings` and `--cap-lints`)
/* FP:lib.rs-0670 */     pub can_emit_warnings: bool,
/* FP:lib.rs-0671 */     /// If Some, the Nth error-level diagnostic is upgraded to bug-level.
/* FP:lib.rs-0672 */     /// (rustc: see `-Z treat-err-as-bug`)
/* FP:lib.rs-0673 */     pub treat_err_as_bug: Option<NonZero<usize>>,
/* FP:lib.rs-0674 */     /// Eagerly emit delayed bugs as errors, so that the compiler debugger may
/* FP:lib.rs-0675 */     /// see all of the errors being emitted at once.
/* FP:lib.rs-0676 */     pub eagerly_emit_delayed_bugs: bool,
/* FP:lib.rs-0677 */     /// Show macro backtraces.
/* FP:lib.rs-0678 */     /// (rustc: see `-Z macro-backtrace`)
/* FP:lib.rs-0679 */     pub macro_backtrace: bool,
/* FP:lib.rs-0680 */     /// If true, identical diagnostics are reported only once.
/* FP:lib.rs-0681 */     pub deduplicate_diagnostics: bool,
/* FP:lib.rs-0682 */     /// Track where errors are created. Enabled with `-Ztrack-diagnostics`.
/* FP:lib.rs-0683 */     pub track_diagnostics: bool,
/* FP:lib.rs-0684 */ }
/* FP:lib.rs-0685 */ 
/* FP:lib.rs-0686 */ impl Drop for DiagCtxtInner {
/* FP:lib.rs-0687 */     fn drop(&mut self) {
/* FP:lib.rs-0688 */         // For tools using `interface::run_compiler` (e.g. rustc, rustdoc)
/* FP:lib.rs-0689 */         // stashed diagnostics will have already been emitted. But for others
/* FP:lib.rs-0690 */         // that don't use `interface::run_compiler` (e.g. rustfmt, some clippy
/* FP:lib.rs-0691 */         // lints) this fallback is necessary.
/* FP:lib.rs-0692 */         //
/* FP:lib.rs-0693 */         // Important: it is sound to produce an `ErrorGuaranteed` when stashing
/* FP:lib.rs-0694 */         // errors because they are guaranteed to be emitted here or earlier.
/* FP:lib.rs-0695 */         self.emit_stashed_diagnostics();
/* FP:lib.rs-0696 */ 
/* FP:lib.rs-0697 */         // Important: it is sound to produce an `ErrorGuaranteed` when emitting
/* FP:lib.rs-0698 */         // delayed bugs because they are guaranteed to be emitted here if
/* FP:lib.rs-0699 */         // necessary.
/* FP:lib.rs-0700 */         self.flush_delayed();
/* FP:lib.rs-0701 */ 
/* FP:lib.rs-0702 */         // Sanity check: did we use some of the expensive `trimmed_def_paths` functions
/* FP:lib.rs-0703 */         // unexpectedly, that is, without producing diagnostics? If so, for debugging purposes, we
/* FP:lib.rs-0704 */         // suggest where this happened and how to avoid it.
/* FP:lib.rs-0705 */         if !self.has_printed && !self.suppressed_expected_diag && !std::thread::panicking() {
/* FP:lib.rs-0706 */             if let Some(backtrace) = &self.must_produce_diag {
/* FP:lib.rs-0707 */                 let suggestion = match backtrace.status() {
/* FP:lib.rs-0708 */                     BacktraceStatus::Disabled => String::from(
/* FP:lib.rs-0709 */                         "Backtraces are currently disabled: set `RUST_BACKTRACE=1` and re-run \
/* FP:lib.rs-0710 */                         to see where it happened.",
/* FP:lib.rs-0711 */                     ),
/* FP:lib.rs-0712 */                     BacktraceStatus::Captured => format!(
/* FP:lib.rs-0713 */                         "This happened in the following `must_produce_diag` call's backtrace:\n\
/* FP:lib.rs-0714 */                         {backtrace}",
/* FP:lib.rs-0715 */                     ),
/* FP:lib.rs-0716 */                     _ => String::from("(impossible to capture backtrace where this happened)"),
/* FP:lib.rs-0717 */                 };
/* FP:lib.rs-0718 */                 panic!(
/* FP:lib.rs-0719 */                     "`trimmed_def_paths` called, diagnostics were expected but none were emitted. \
/* FP:lib.rs-0720 */                     Use `with_no_trimmed_paths` for debugging. {suggestion}"
/* FP:lib.rs-0721 */                 );
/* FP:lib.rs-0722 */             }
/* FP:lib.rs-0723 */         }
/* FP:lib.rs-0724 */     }
/* FP:lib.rs-0725 */ }
/* FP:lib.rs-0726 */ 
/* FP:lib.rs-0727 */ impl DiagCtxt {
/* FP:lib.rs-0728 */     pub fn disable_warnings(mut self) -> Self {
/* FP:lib.rs-0729 */         self.inner.get_mut().flags.can_emit_warnings = false;
/* FP:lib.rs-0730 */         self
/* FP:lib.rs-0731 */     }
/* FP:lib.rs-0732 */ 
/* FP:lib.rs-0733 */     pub fn with_flags(mut self, flags: DiagCtxtFlags) -> Self {
/* FP:lib.rs-0734 */         self.inner.get_mut().flags = flags;
/* FP:lib.rs-0735 */         self
/* FP:lib.rs-0736 */     }
/* FP:lib.rs-0737 */ 
/* FP:lib.rs-0738 */     pub fn with_ice_file(mut self, ice_file: PathBuf) -> Self {
/* FP:lib.rs-0739 */         self.inner.get_mut().ice_file = Some(ice_file);
/* FP:lib.rs-0740 */         self
/* FP:lib.rs-0741 */     }
/* FP:lib.rs-0742 */ 
/* FP:lib.rs-0743 */     pub fn with_registry(mut self, registry: Registry) -> Self {
/* FP:lib.rs-0744 */         self.inner.get_mut().registry = registry;
/* FP:lib.rs-0745 */         self
/* FP:lib.rs-0746 */     }
/* FP:lib.rs-0747 */ 
/* FP:lib.rs-0748 */     pub fn new(emitter: Box<DynEmitter>) -> Self {
/* FP:lib.rs-0749 */         Self { inner: Lock::new(DiagCtxtInner::new(emitter)) }
/* FP:lib.rs-0750 */     }
/* FP:lib.rs-0751 */ 
/* FP:lib.rs-0752 */     pub fn make_silent(&self) {
/* FP:lib.rs-0753 */         let mut inner = self.inner.borrow_mut();
/* FP:lib.rs-0754 */         let translator = inner.emitter.translator().clone();
/* FP:lib.rs-0755 */         inner.emitter = Box::new(emitter::SilentEmitter { translator });
/* FP:lib.rs-0756 */     }
/* FP:lib.rs-0757 */ 
/* FP:lib.rs-0758 */     pub fn set_emitter(&self, emitter: Box<dyn Emitter + DynSend>) {
/* FP:lib.rs-0759 */         self.inner.borrow_mut().emitter = emitter;
/* FP:lib.rs-0760 */     }
/* FP:lib.rs-0761 */ 
/* FP:lib.rs-0762 */     /// Translate `message` eagerly with `args` to `SubdiagMessage::Eager`.
/* FP:lib.rs-0763 */     pub fn eagerly_translate<'a>(
/* FP:lib.rs-0764 */         &self,
/* FP:lib.rs-0765 */         message: DiagMessage,
/* FP:lib.rs-0766 */         args: impl Iterator<Item = DiagArg<'a>>,
/* FP:lib.rs-0767 */     ) -> SubdiagMessage {
/* FP:lib.rs-0768 */         let inner = self.inner.borrow();
/* FP:lib.rs-0769 */         inner.eagerly_translate(message, args)
/* FP:lib.rs-0770 */     }
/* FP:lib.rs-0771 */ 
/* FP:lib.rs-0772 */     /// Translate `message` eagerly with `args` to `String`.
/* FP:lib.rs-0773 */     pub fn eagerly_translate_to_string<'a>(
/* FP:lib.rs-0774 */         &self,
/* FP:lib.rs-0775 */         message: DiagMessage,
/* FP:lib.rs-0776 */         args: impl Iterator<Item = DiagArg<'a>>,
/* FP:lib.rs-0777 */     ) -> String {
/* FP:lib.rs-0778 */         let inner = self.inner.borrow();
/* FP:lib.rs-0779 */         inner.eagerly_translate_to_string(message, args)
/* FP:lib.rs-0780 */     }
/* FP:lib.rs-0781 */ 
/* FP:lib.rs-0782 */     // This is here to not allow mutation of flags;
/* FP:lib.rs-0783 */     // as of this writing it's used in Session::consider_optimizing and
/* FP:lib.rs-0784 */     // in tests in rustc_interface.
/* FP:lib.rs-0785 */     pub fn can_emit_warnings(&self) -> bool {
/* FP:lib.rs-0786 */         self.inner.borrow_mut().flags.can_emit_warnings
/* FP:lib.rs-0787 */     }
/* FP:lib.rs-0788 */ 
/* FP:lib.rs-0789 */     /// Resets the diagnostic error count as well as the cached emitted diagnostics.
/* FP:lib.rs-0790 */     ///
/* FP:lib.rs-0791 */     /// NOTE: *do not* call this function from rustc. It is only meant to be called from external
/* FP:lib.rs-0792 */     /// tools that want to reuse a `Parser` cleaning the previously emitted diagnostics as well as
/* FP:lib.rs-0793 */     /// the overall count of emitted error diagnostics.
/* FP:lib.rs-0794 */     pub fn reset_err_count(&self) {
/* FP:lib.rs-0795 */         // Use destructuring so that if a field gets added to `DiagCtxtInner`, it's impossible to
/* FP:lib.rs-0796 */         // fail to update this method as well.
/* FP:lib.rs-0797 */         let mut inner = self.inner.borrow_mut();
/* FP:lib.rs-0798 */         let DiagCtxtInner {
/* FP:lib.rs-0799 */             flags: _,
/* FP:lib.rs-0800 */             registry: _,
/* FP:lib.rs-0801 */             err_guars,
/* FP:lib.rs-0802 */             lint_err_guars,
/* FP:lib.rs-0803 */             delayed_bugs,
/* FP:lib.rs-0804 */             deduplicated_err_count,
/* FP:lib.rs-0805 */             deduplicated_warn_count,
/* FP:lib.rs-0806 */             emitter: _,
/* FP:lib.rs-0807 */             must_produce_diag,
/* FP:lib.rs-0808 */             has_printed,
/* FP:lib.rs-0809 */             suppressed_expected_diag,
/* FP:lib.rs-0810 */             taught_diagnostics,
/* FP:lib.rs-0811 */             emitted_diagnostic_codes,
/* FP:lib.rs-0812 */             emitted_diagnostics,
/* FP:lib.rs-0813 */             stashed_diagnostics,
/* FP:lib.rs-0814 */             future_breakage_diagnostics,
/* FP:lib.rs-0815 */             fulfilled_expectations,
/* FP:lib.rs-0816 */             ice_file: _,
/* FP:lib.rs-0817 */         } = inner.deref_mut();
/* FP:lib.rs-0818 */ 
/* FP:lib.rs-0819 */         // For the `Vec`s and `HashMap`s, we overwrite with an empty container to free the
/* FP:lib.rs-0820 */         // underlying memory (which `clear` would not do).
/* FP:lib.rs-0821 */         *err_guars = Default::default();
/* FP:lib.rs-0822 */         *lint_err_guars = Default::default();
/* FP:lib.rs-0823 */         *delayed_bugs = Default::default();
/* FP:lib.rs-0824 */         *deduplicated_err_count = 0;
/* FP:lib.rs-0825 */         *deduplicated_warn_count = 0;
/* FP:lib.rs-0826 */         *must_produce_diag = None;
/* FP:lib.rs-0827 */         *has_printed = false;
/* FP:lib.rs-0828 */         *suppressed_expected_diag = false;
/* FP:lib.rs-0829 */         *taught_diagnostics = Default::default();
/* FP:lib.rs-0830 */         *emitted_diagnostic_codes = Default::default();
/* FP:lib.rs-0831 */         *emitted_diagnostics = Default::default();
/* FP:lib.rs-0832 */         *stashed_diagnostics = Default::default();
/* FP:lib.rs-0833 */         *future_breakage_diagnostics = Default::default();
/* FP:lib.rs-0834 */         *fulfilled_expectations = Default::default();
/* FP:lib.rs-0835 */     }
/* FP:lib.rs-0836 */ 
/* FP:lib.rs-0837 */     pub fn handle<'a>(&'a self) -> DiagCtxtHandle<'a> {
/* FP:lib.rs-0838 */         DiagCtxtHandle { dcx: self, tainted_with_errors: None }
/* FP:lib.rs-0839 */     }
/* FP:lib.rs-0840 */ 
/* FP:lib.rs-0841 */     /// Link this to a taintable context so that emitting errors will automatically set
/* FP:lib.rs-0842 */     /// the `Option<ErrorGuaranteed>` instead of having to do that manually at every error
/* FP:lib.rs-0843 */     /// emission site.
/* FP:lib.rs-0844 */     pub fn taintable_handle<'a>(
/* FP:lib.rs-0845 */         &'a self,
/* FP:lib.rs-0846 */         tainted_with_errors: &'a Cell<Option<ErrorGuaranteed>>,
/* FP:lib.rs-0847 */     ) -> DiagCtxtHandle<'a> {
/* FP:lib.rs-0848 */         DiagCtxtHandle { dcx: self, tainted_with_errors: Some(tainted_with_errors) }
/* FP:lib.rs-0849 */     }
/* FP:lib.rs-0850 */ }
/* FP:lib.rs-0851 */ 
/* FP:lib.rs-0852 */ impl<'a> DiagCtxtHandle<'a> {
/* FP:lib.rs-0853 */     /// Stashes a diagnostic for possible later improvement in a different,
/* FP:lib.rs-0854 */     /// later stage of the compiler. Possible actions depend on the diagnostic
/* FP:lib.rs-0855 */     /// level:
/* FP:lib.rs-0856 */     /// - Level::Bug, Level:Fatal: not allowed, will trigger a panic.
/* FP:lib.rs-0857 */     /// - Level::Error: immediately counted as an error that has occurred, because it
/* FP:lib.rs-0858 */     ///   is guaranteed to be emitted eventually. Can be later accessed with the
/* FP:lib.rs-0859 */     ///   provided `span` and `key` through
/* FP:lib.rs-0860 */     ///   [`DiagCtxtHandle::try_steal_modify_and_emit_err`] or
/* FP:lib.rs-0861 */     ///   [`DiagCtxtHandle::try_steal_replace_and_emit_err`]. These do not allow
/* FP:lib.rs-0862 */     ///   cancellation or downgrading of the error. Returns
/* FP:lib.rs-0863 */     ///   `Some(ErrorGuaranteed)`.
/* FP:lib.rs-0864 */     /// - Level::DelayedBug: this does happen occasionally with errors that are
/* FP:lib.rs-0865 */     ///   downgraded to delayed bugs. It is not stashed, but immediately
/* FP:lib.rs-0866 */     ///   emitted as a delayed bug. This is because stashing it would cause it
/* FP:lib.rs-0867 */     ///   to be counted by `err_count` which we don't want. It doesn't matter
/* FP:lib.rs-0868 */     ///   that we cannot steal and improve it later, because it's not a
/* FP:lib.rs-0869 */     ///   user-facing error. Returns `Some(ErrorGuaranteed)` as is normal for
/* FP:lib.rs-0870 */     ///   delayed bugs.
/* FP:lib.rs-0871 */     /// - Level::Warning and lower (i.e. !is_error()): can be accessed with the
/* FP:lib.rs-0872 */     ///   provided `span` and `key` through [`DiagCtxtHandle::steal_non_err()`]. This
/* FP:lib.rs-0873 */     ///   allows cancelling and downgrading of the diagnostic. Returns `None`.
/* FP:lib.rs-0874 */     pub fn stash_diagnostic(
/* FP:lib.rs-0875 */         &self,
/* FP:lib.rs-0876 */         span: Span,
/* FP:lib.rs-0877 */         key: StashKey,
/* FP:lib.rs-0878 */         diag: DiagInner,
/* FP:lib.rs-0879 */     ) -> Option<ErrorGuaranteed> {
/* FP:lib.rs-0880 */         let guar = match diag.level {
/* FP:lib.rs-0881 */             Bug | Fatal => {
/* FP:lib.rs-0882 */                 self.span_bug(
/* FP:lib.rs-0883 */                     span,
/* FP:lib.rs-0884 */                     format!("invalid level in `stash_diagnostic`: {:?}", diag.level),
/* FP:lib.rs-0885 */                 );
/* FP:lib.rs-0886 */             }
/* FP:lib.rs-0887 */             // We delay a bug here so that `-Ztreat-err-as-bug -Zeagerly-emit-delayed-bugs`
/* FP:lib.rs-0888 */             // can be used to create a backtrace at the stashing site instead of whenever the
/* FP:lib.rs-0889 */             // diagnostic context is dropped and thus delayed bugs are emitted.
/* FP:lib.rs-0890 */             Error => Some(self.span_delayed_bug(span, format!("stashing {key:?}"))),
/* FP:lib.rs-0891 */             DelayedBug => {
/* FP:lib.rs-0892 */                 return self.inner.borrow_mut().emit_diagnostic(diag, self.tainted_with_errors);
/* FP:lib.rs-0893 */             }
/* FP:lib.rs-0894 */             ForceWarning | Warning | Note | OnceNote | Help | OnceHelp | FailureNote | Allow
/* FP:lib.rs-0895 */             | Expect => None,
/* FP:lib.rs-0896 */         };
/* FP:lib.rs-0897 */ 
/* FP:lib.rs-0898 */         // FIXME(Centril, #69537): Consider reintroducing panic on overwriting a stashed diagnostic
/* FP:lib.rs-0899 */         // if/when we have a more robust macro-friendly replacement for `(span, key)` as a key.
/* FP:lib.rs-0900 */         // See the PR for a discussion.
/* FP:lib.rs-0901 */         self.inner
/* FP:lib.rs-0902 */             .borrow_mut()
/* FP:lib.rs-0903 */             .stashed_diagnostics
/* FP:lib.rs-0904 */             .entry(key)
/* FP:lib.rs-0905 */             .or_default()
/* FP:lib.rs-0906 */             .insert(span.with_parent(None), (diag, guar));
/* FP:lib.rs-0907 */ 
/* FP:lib.rs-0908 */         guar
/* FP:lib.rs-0909 */     }
/* FP:lib.rs-0910 */ 
/* FP:lib.rs-0911 */     /// Steal a previously stashed non-error diagnostic with the given `Span`
/* FP:lib.rs-0912 */     /// and [`StashKey`] as the key. Panics if the found diagnostic is an
/* FP:lib.rs-0913 */     /// error.
/* FP:lib.rs-0914 */     pub fn steal_non_err(self, span: Span, key: StashKey) -> Option<Diag<'a, ()>> {
/* FP:lib.rs-0915 */         // FIXME(#120456) - is `swap_remove` correct?
/* FP:lib.rs-0916 */         let (diag, guar) = self.inner.borrow_mut().stashed_diagnostics.get_mut(&key).and_then(
/* FP:lib.rs-0917 */             |stashed_diagnostics| stashed_diagnostics.swap_remove(&span.with_parent(None)),
/* FP:lib.rs-0918 */         )?;
/* FP:lib.rs-0919 */         assert!(!diag.is_error());
/* FP:lib.rs-0920 */         assert!(guar.is_none());
/* FP:lib.rs-0921 */         Some(Diag::new_diagnostic(self, diag))
/* FP:lib.rs-0922 */     }
/* FP:lib.rs-0923 */ 
/* FP:lib.rs-0924 */     /// Steals a previously stashed error with the given `Span` and
/* FP:lib.rs-0925 */     /// [`StashKey`] as the key, modifies it, and emits it. Returns `None` if
/* FP:lib.rs-0926 */     /// no matching diagnostic is found. Panics if the found diagnostic's level
/* FP:lib.rs-0927 */     /// isn't `Level::Error`.
/* FP:lib.rs-0928 */     pub fn try_steal_modify_and_emit_err<F>(
/* FP:lib.rs-0929 */         self,
/* FP:lib.rs-0930 */         span: Span,
/* FP:lib.rs-0931 */         key: StashKey,
/* FP:lib.rs-0932 */         mut modify_err: F,
/* FP:lib.rs-0933 */     ) -> Option<ErrorGuaranteed>
/* FP:lib.rs-0934 */     where
/* FP:lib.rs-0935 */         F: FnMut(&mut Diag<'_>),
/* FP:lib.rs-0936 */     {
/* FP:lib.rs-0937 */         // FIXME(#120456) - is `swap_remove` correct?
/* FP:lib.rs-0938 */         let err = self.inner.borrow_mut().stashed_diagnostics.get_mut(&key).and_then(
/* FP:lib.rs-0939 */             |stashed_diagnostics| stashed_diagnostics.swap_remove(&span.with_parent(None)),
/* FP:lib.rs-0940 */         );
/* FP:lib.rs-0941 */         err.map(|(err, guar)| {
/* FP:lib.rs-0942 */             // The use of `::<ErrorGuaranteed>` is safe because level is `Level::Error`.
/* FP:lib.rs-0943 */             assert_eq!(err.level, Error);
/* FP:lib.rs-0944 */             assert!(guar.is_some());
/* FP:lib.rs-0945 */             let mut err = Diag::<ErrorGuaranteed>::new_diagnostic(self, err);
/* FP:lib.rs-0946 */             modify_err(&mut err);
/* FP:lib.rs-0947 */             assert_eq!(err.level, Error);
/* FP:lib.rs-0948 */             err.emit()
/* FP:lib.rs-0949 */         })
/* FP:lib.rs-0950 */     }
/* FP:lib.rs-0951 */ 
/* FP:lib.rs-0952 */     /// Steals a previously stashed error with the given `Span` and
/* FP:lib.rs-0953 */     /// [`StashKey`] as the key, cancels it if found, and emits `new_err`.
/* FP:lib.rs-0954 */     /// Panics if the found diagnostic's level isn't `Level::Error`.
/* FP:lib.rs-0955 */     pub fn try_steal_replace_and_emit_err(
/* FP:lib.rs-0956 */         self,
/* FP:lib.rs-0957 */         span: Span,
/* FP:lib.rs-0958 */         key: StashKey,
/* FP:lib.rs-0959 */         new_err: Diag<'_>,
/* FP:lib.rs-0960 */     ) -> ErrorGuaranteed {
/* FP:lib.rs-0961 */         // FIXME(#120456) - is `swap_remove` correct?
/* FP:lib.rs-0962 */         let old_err = self.inner.borrow_mut().stashed_diagnostics.get_mut(&key).and_then(
/* FP:lib.rs-0963 */             |stashed_diagnostics| stashed_diagnostics.swap_remove(&span.with_parent(None)),
/* FP:lib.rs-0964 */         );
/* FP:lib.rs-0965 */         match old_err {
/* FP:lib.rs-0966 */             Some((old_err, guar)) => {
/* FP:lib.rs-0967 */                 assert_eq!(old_err.level, Error);
/* FP:lib.rs-0968 */                 assert!(guar.is_some());
/* FP:lib.rs-0969 */                 // Because `old_err` has already been counted, it can only be
/* FP:lib.rs-0970 */                 // safely cancelled because the `new_err` supplants it.
/* FP:lib.rs-0971 */                 Diag::<ErrorGuaranteed>::new_diagnostic(self, old_err).cancel();
/* FP:lib.rs-0972 */             }
/* FP:lib.rs-0973 */             None => {}
/* FP:lib.rs-0974 */         };
/* FP:lib.rs-0975 */         new_err.emit()
/* FP:lib.rs-0976 */     }
/* FP:lib.rs-0977 */ 
/* FP:lib.rs-0978 */     pub fn has_stashed_diagnostic(&self, span: Span, key: StashKey) -> bool {
/* FP:lib.rs-0979 */         let inner = self.inner.borrow();
/* FP:lib.rs-0980 */         if let Some(stashed_diagnostics) = inner.stashed_diagnostics.get(&key)
/* FP:lib.rs-0981 */             && !stashed_diagnostics.is_empty()
/* FP:lib.rs-0982 */         {
/* FP:lib.rs-0983 */             stashed_diagnostics.contains_key(&span.with_parent(None))
/* FP:lib.rs-0984 */         } else {
/* FP:lib.rs-0985 */             false
/* FP:lib.rs-0986 */         }
/* FP:lib.rs-0987 */     }
/* FP:lib.rs-0988 */ 
/* FP:lib.rs-0989 */     /// Emit all stashed diagnostics.
/* FP:lib.rs-0990 */     pub fn emit_stashed_diagnostics(&self) -> Option<ErrorGuaranteed> {
/* FP:lib.rs-0991 */         self.inner.borrow_mut().emit_stashed_diagnostics()
/* FP:lib.rs-0992 */     }
/* FP:lib.rs-0993 */ 
/* FP:lib.rs-0994 */     /// This excludes delayed bugs.
/* FP:lib.rs-0995 */     #[inline]
/* FP:lib.rs-0996 */     pub fn err_count(&self) -> usize {
/* FP:lib.rs-0997 */         let inner = self.inner.borrow();
/* FP:lib.rs-0998 */         inner.err_guars.len()
/* FP:lib.rs-0999 */             + inner.lint_err_guars.len()
/* FP:lib.rs-1000 */             + inner
/* FP:lib.rs-1001 */                 .stashed_diagnostics
/* FP:lib.rs-1002 */                 .values()
/* FP:lib.rs-1003 */                 .map(|a| a.values().filter(|(_, guar)| guar.is_some()).count())
/* FP:lib.rs-1004 */                 .sum::<usize>()
/* FP:lib.rs-1005 */     }
/* FP:lib.rs-1006 */ 
/* FP:lib.rs-1007 */     /// This excludes lint errors and delayed bugs. Unless absolutely
/* FP:lib.rs-1008 */     /// necessary, prefer `has_errors` to this method.
/* FP:lib.rs-1009 */     pub fn has_errors_excluding_lint_errors(&self) -> Option<ErrorGuaranteed> {
/* FP:lib.rs-1010 */         self.inner.borrow().has_errors_excluding_lint_errors()
/* FP:lib.rs-1011 */     }
/* FP:lib.rs-1012 */ 
/* FP:lib.rs-1013 */     /// This excludes delayed bugs.
/* FP:lib.rs-1014 */     pub fn has_errors(&self) -> Option<ErrorGuaranteed> {
/* FP:lib.rs-1015 */         self.inner.borrow().has_errors()
/* FP:lib.rs-1016 */     }
/* FP:lib.rs-1017 */ 
/* FP:lib.rs-1018 */     /// This excludes nothing. Unless absolutely necessary, prefer `has_errors`
/* FP:lib.rs-1019 */     /// to this method.
/* FP:lib.rs-1020 */     pub fn has_errors_or_delayed_bugs(&self) -> Option<ErrorGuaranteed> {
/* FP:lib.rs-1021 */         self.inner.borrow().has_errors_or_delayed_bugs()
/* FP:lib.rs-1022 */     }
/* FP:lib.rs-1023 */ 
/* FP:lib.rs-1024 */     pub fn print_error_count(&self) {
/* FP:lib.rs-1025 */         let mut inner = self.inner.borrow_mut();
/* FP:lib.rs-1026 */ 
/* FP:lib.rs-1027 */         // Any stashed diagnostics should have been handled by
/* FP:lib.rs-1028 */         // `emit_stashed_diagnostics` by now.
/* FP:lib.rs-1029 */         assert!(inner.stashed_diagnostics.is_empty());
/* FP:lib.rs-1030 */ 
/* FP:lib.rs-1031 */         if inner.treat_err_as_bug() {
/* FP:lib.rs-1032 */             return;
/* FP:lib.rs-1033 */         }
/* FP:lib.rs-1034 */ 
/* FP:lib.rs-1035 */         let warnings = match inner.deduplicated_warn_count {
/* FP:lib.rs-1036 */             0 => Cow::from(""),
/* FP:lib.rs-1037 */             1 => Cow::from("1 warning emitted"),
/* FP:lib.rs-1038 */             count => Cow::from(format!("{count} warnings emitted")),
/* FP:lib.rs-1039 */         };
/* FP:lib.rs-1040 */         let errors = match inner.deduplicated_err_count {
/* FP:lib.rs-1041 */             0 => Cow::from(""),
/* FP:lib.rs-1042 */             1 => Cow::from("aborting due to 1 previous error"),
/* FP:lib.rs-1043 */             count => Cow::from(format!("aborting due to {count} previous errors")),
/* FP:lib.rs-1044 */         };
/* FP:lib.rs-1045 */ 
/* FP:lib.rs-1046 */         match (errors.len(), warnings.len()) {
/* FP:lib.rs-1047 */             (0, 0) => return,
/* FP:lib.rs-1048 */             (0, _) => {
/* FP:lib.rs-1049 */                 // Use `ForceWarning` rather than `Warning` to guarantee emission, e.g. with a
/* FP:lib.rs-1050 */                 // configuration like `--cap-lints allow --force-warn bare_trait_objects`.
/* FP:lib.rs-1051 */                 inner.emit_diagnostic(
/* FP:lib.rs-1052 */                     DiagInner::new(ForceWarning, DiagMessage::Str(warnings)),
/* FP:lib.rs-1053 */                     None,
/* FP:lib.rs-1054 */                 );
/* FP:lib.rs-1055 */             }
/* FP:lib.rs-1056 */             (_, 0) => {
/* FP:lib.rs-1057 */                 inner.emit_diagnostic(DiagInner::new(Error, errors), self.tainted_with_errors);
/* FP:lib.rs-1058 */             }
/* FP:lib.rs-1059 */             (_, _) => {
/* FP:lib.rs-1060 */                 inner.emit_diagnostic(
/* FP:lib.rs-1061 */                     DiagInner::new(Error, format!("{errors}; {warnings}")),
/* FP:lib.rs-1062 */                     self.tainted_with_errors,
/* FP:lib.rs-1063 */                 );
/* FP:lib.rs-1064 */             }
/* FP:lib.rs-1065 */         }
/* FP:lib.rs-1066 */ 
/* FP:lib.rs-1067 */         let can_show_explain = inner.emitter.should_show_explain();
/* FP:lib.rs-1068 */         let are_there_diagnostics = !inner.emitted_diagnostic_codes.is_empty();
/* FP:lib.rs-1069 */         if can_show_explain && are_there_diagnostics {
/* FP:lib.rs-1070 */             let mut error_codes = inner
/* FP:lib.rs-1071 */                 .emitted_diagnostic_codes
/* FP:lib.rs-1072 */                 .iter()
/* FP:lib.rs-1073 */                 .filter_map(|&code| {
/* FP:lib.rs-1074 */                     if inner.registry.try_find_description(code).is_ok() {
/* FP:lib.rs-1075 */                         Some(code.to_string())
/* FP:lib.rs-1076 */                     } else {
/* FP:lib.rs-1077 */                         None
/* FP:lib.rs-1078 */                     }
/* FP:lib.rs-1079 */                 })
/* FP:lib.rs-1080 */                 .collect::<Vec<_>>();
/* FP:lib.rs-1081 */             if !error_codes.is_empty() {
/* FP:lib.rs-1082 */                 error_codes.sort();
/* FP:lib.rs-1083 */                 if error_codes.len() > 1 {
/* FP:lib.rs-1084 */                     let limit = if error_codes.len() > 9 { 9 } else { error_codes.len() };
/* FP:lib.rs-1085 */                     let msg1 = format!(
/* FP:lib.rs-1086 */                         "Some errors have detailed explanations: {}{}",
/* FP:lib.rs-1087 */                         error_codes[..limit].join(", "),
/* FP:lib.rs-1088 */                         if error_codes.len() > 9 { "..." } else { "." }
/* FP:lib.rs-1089 */                     );
/* FP:lib.rs-1090 */                     let msg2 = format!(
/* FP:lib.rs-1091 */                         "For more information about an error, try `rustc --explain {}`.",
/* FP:lib.rs-1092 */                         &error_codes[0]
/* FP:lib.rs-1093 */                     );
/* FP:lib.rs-1094 */                     inner.emit_diagnostic(DiagInner::new(FailureNote, msg1), None);
/* FP:lib.rs-1095 */                     inner.emit_diagnostic(DiagInner::new(FailureNote, msg2), None);
/* FP:lib.rs-1096 */                 } else {
/* FP:lib.rs-1097 */                     let msg = format!(
/* FP:lib.rs-1098 */                         "For more information about this error, try `rustc --explain {}`.",
/* FP:lib.rs-1099 */                         &error_codes[0]
/* FP:lib.rs-1100 */                     );
/* FP:lib.rs-1101 */                     inner.emit_diagnostic(DiagInner::new(FailureNote, msg), None);
/* FP:lib.rs-1102 */                 }
/* FP:lib.rs-1103 */             }
/* FP:lib.rs-1104 */         }
/* FP:lib.rs-1105 */     }
/* FP:lib.rs-1106 */ 
/* FP:lib.rs-1107 */     /// This excludes delayed bugs. Used for early aborts after errors occurred
/* FP:lib.rs-1108 */     /// -- e.g. because continuing in the face of errors is likely to lead to
/* FP:lib.rs-1109 */     /// bad results, such as spurious/uninteresting additional errors -- when
/* FP:lib.rs-1110 */     /// returning an error `Result` is difficult.
/* FP:lib.rs-1111 */     pub fn abort_if_errors(&self) {
/* FP:lib.rs-1112 */         if let Some(guar) = self.has_errors() {
/* FP:lib.rs-1113 */             guar.raise_fatal();
/* FP:lib.rs-1114 */         }
/* FP:lib.rs-1115 */     }
/* FP:lib.rs-1116 */ 
/* FP:lib.rs-1117 */     /// `true` if we haven't taught a diagnostic with this code already.
/* FP:lib.rs-1118 */     /// The caller must then teach the user about such a diagnostic.
/* FP:lib.rs-1119 */     ///
/* FP:lib.rs-1120 */     /// Used to suppress emitting the same error multiple times with extended explanation when
/* FP:lib.rs-1121 */     /// calling `-Zteach`.
/* FP:lib.rs-1122 */     pub fn must_teach(&self, code: ErrCode) -> bool {
/* FP:lib.rs-1123 */         self.inner.borrow_mut().taught_diagnostics.insert(code)
/* FP:lib.rs-1124 */     }
/* FP:lib.rs-1125 */ 
/* FP:lib.rs-1126 */     pub fn emit_diagnostic(&self, diagnostic: DiagInner) -> Option<ErrorGuaranteed> {
/* FP:lib.rs-1127 */         self.inner.borrow_mut().emit_diagnostic(diagnostic, self.tainted_with_errors)
/* FP:lib.rs-1128 */     }
/* FP:lib.rs-1129 */ 
/* FP:lib.rs-1130 */     pub fn emit_artifact_notification(&self, path: &Path, artifact_type: &str) {
/* FP:lib.rs-1131 */         self.inner.borrow_mut().emitter.emit_artifact_notification(path, artifact_type);
/* FP:lib.rs-1132 */     }
/* FP:lib.rs-1133 */ 
/* FP:lib.rs-1134 */     pub fn emit_timing_section_start(&self, record: TimingRecord) {
/* FP:lib.rs-1135 */         self.inner.borrow_mut().emitter.emit_timing_section(record, TimingEvent::Start);
/* FP:lib.rs-1136 */     }
/* FP:lib.rs-1137 */ 
/* FP:lib.rs-1138 */     pub fn emit_timing_section_end(&self, record: TimingRecord) {
/* FP:lib.rs-1139 */         self.inner.borrow_mut().emitter.emit_timing_section(record, TimingEvent::End);
/* FP:lib.rs-1140 */     }
/* FP:lib.rs-1141 */ 
/* FP:lib.rs-1142 */     pub fn emit_future_breakage_report(&self) {
/* FP:lib.rs-1143 */         let inner = &mut *self.inner.borrow_mut();
/* FP:lib.rs-1144 */         let diags = std::mem::take(&mut inner.future_breakage_diagnostics);
/* FP:lib.rs-1145 */         if !diags.is_empty() {
/* FP:lib.rs-1146 */             inner.emitter.emit_future_breakage_report(diags, &inner.registry);
/* FP:lib.rs-1147 */         }
/* FP:lib.rs-1148 */     }
/* FP:lib.rs-1149 */ 
/* FP:lib.rs-1150 */     pub fn emit_unused_externs(
/* FP:lib.rs-1151 */         &self,
/* FP:lib.rs-1152 */         lint_level: crate::rustc_lint_defs::Level,
/* FP:lib.rs-1153 */         loud: bool,
/* FP:lib.rs-1154 */         unused_externs: &[&str],
/* FP:lib.rs-1155 */     ) {
/* FP:lib.rs-1156 */         let mut inner = self.inner.borrow_mut();
/* FP:lib.rs-1157 */ 
/* FP:lib.rs-1158 */         // This "error" is an odd duck.
/* FP:lib.rs-1159 */         // - It's only produce with JSON output.
/* FP:lib.rs-1160 */         // - It's not emitted the usual way, via `emit_diagnostic`.
/* FP:lib.rs-1161 */         // - The `$message_type` field is "unused_externs" rather than the usual
/* FP:lib.rs-1162 */         //   "diagnostic".
/* FP:lib.rs-1163 */         //
/* FP:lib.rs-1164 */         // We count it as a lint error because it has a lint level. The value
/* FP:lib.rs-1165 */         // of `loud` (which comes from "unused-externs" or
/* FP:lib.rs-1166 */         // "unused-externs-silent"), also affects whether it's treated like a
/* FP:lib.rs-1167 */         // hard error or not.
/* FP:lib.rs-1168 */         if loud && lint_level.is_error() {
/* FP:lib.rs-1169 */             // This `unchecked_error_guaranteed` is valid. It is where the
/* FP:lib.rs-1170 */             // `ErrorGuaranteed` for unused_extern errors originates.
/* FP:lib.rs-1171 */             #[allow(deprecated)]
/* FP:lib.rs-1172 */             inner.lint_err_guars.push(ErrorGuaranteed::unchecked_error_guaranteed());
/* FP:lib.rs-1173 */             inner.panic_if_treat_err_as_bug();
/* FP:lib.rs-1174 */         }
/* FP:lib.rs-1175 */ 
/* FP:lib.rs-1176 */         inner.emitter.emit_unused_externs(lint_level, unused_externs)
/* FP:lib.rs-1177 */     }
/* FP:lib.rs-1178 */ 
/* FP:lib.rs-1179 */     /// This methods steals all [`LintExpectationId`]s that are stored inside
/* FP:lib.rs-1180 */     /// [`DiagCtxtInner`] and indicate that the linked expectation has been fulfilled.
/* FP:lib.rs-1181 */     #[must_use]
/* FP:lib.rs-1182 */     pub fn steal_fulfilled_expectation_ids(&self) -> FxIndexSet<LintExpectationId> {
/* FP:lib.rs-1183 */         std::mem::take(&mut self.inner.borrow_mut().fulfilled_expectations)
/* FP:lib.rs-1184 */     }
/* FP:lib.rs-1185 */ 
/* FP:lib.rs-1186 */     pub fn flush_delayed(&self) {
/* FP:lib.rs-1187 */         self.inner.borrow_mut().flush_delayed();
/* FP:lib.rs-1188 */     }
/* FP:lib.rs-1189 */ 
/* FP:lib.rs-1190 */     /// Used when trimmed_def_paths is called and we must produce a diagnostic
/* FP:lib.rs-1191 */     /// to justify its cost.
/* FP:lib.rs-1192 */     #[track_caller]
/* FP:lib.rs-1193 */     pub fn set_must_produce_diag(&self) {
/* FP:lib.rs-1194 */         assert!(
/* FP:lib.rs-1195 */             self.inner.borrow().must_produce_diag.is_none(),
/* FP:lib.rs-1196 */             "should only need to collect a backtrace once"
/* FP:lib.rs-1197 */         );
/* FP:lib.rs-1198 */         self.inner.borrow_mut().must_produce_diag = Some(Backtrace::capture());
/* FP:lib.rs-1199 */     }
/* FP:lib.rs-1200 */ }
/* FP:lib.rs-1201 */ 
/* FP:lib.rs-1202 */ // This `impl` block contains only the public diagnostic creation/emission API.
/* FP:lib.rs-1203 */ //
/* FP:lib.rs-1204 */ // Functions beginning with `struct_`/`create_` create a diagnostic. Other
/* FP:lib.rs-1205 */ // functions create and emit a diagnostic all in one go.
/* FP:lib.rs-1206 */ impl<'a> DiagCtxtHandle<'a> {
/* FP:lib.rs-1207 */     // No `#[rustc_lint_diagnostics]` and no `impl Into<DiagMessage>` because bug messages aren't
/* FP:lib.rs-1208 */     // user-facing.
/* FP:lib.rs-1209 */     #[track_caller]
/* FP:lib.rs-1210 */     pub fn struct_bug(self, msg: impl Into<Cow<'static, str>>) -> Diag<'a, BugAbort> {
/* FP:lib.rs-1211 */         Diag::new(self, Bug, msg.into())
/* FP:lib.rs-1212 */     }
/* FP:lib.rs-1213 */ 
/* FP:lib.rs-1214 */     // No `#[rustc_lint_diagnostics]` and no `impl Into<DiagMessage>` because bug messages aren't
/* FP:lib.rs-1215 */     // user-facing.
/* FP:lib.rs-1216 */     #[track_caller]
/* FP:lib.rs-1217 */     pub fn bug(self, msg: impl Into<Cow<'static, str>>) -> ! {
/* FP:lib.rs-1218 */         self.struct_bug(msg).emit()
/* FP:lib.rs-1219 */     }
/* FP:lib.rs-1220 */ 
/* FP:lib.rs-1221 */     // No `#[rustc_lint_diagnostics]` and no `impl Into<DiagMessage>` because bug messages aren't
/* FP:lib.rs-1222 */     // user-facing.
/* FP:lib.rs-1223 */     #[track_caller]
/* FP:lib.rs-1224 */     pub fn struct_span_bug(
/* FP:lib.rs-1225 */         self,
/* FP:lib.rs-1226 */         span: impl Into<MultiSpan>,
/* FP:lib.rs-1227 */         msg: impl Into<Cow<'static, str>>,
/* FP:lib.rs-1228 */     ) -> Diag<'a, BugAbort> {
/* FP:lib.rs-1229 */         self.struct_bug(msg).with_span(span)
/* FP:lib.rs-1230 */     }
/* FP:lib.rs-1231 */ 
/* FP:lib.rs-1232 */     // No `#[rustc_lint_diagnostics]` and no `impl Into<DiagMessage>` because bug messages aren't
/* FP:lib.rs-1233 */     // user-facing.
/* FP:lib.rs-1234 */     #[track_caller]
/* FP:lib.rs-1235 */     pub fn span_bug(self, span: impl Into<MultiSpan>, msg: impl Into<Cow<'static, str>>) -> ! {
/* FP:lib.rs-1236 */         self.struct_span_bug(span, msg.into()).emit()
/* FP:lib.rs-1237 */     }
/* FP:lib.rs-1238 */ 
/* FP:lib.rs-1239 */     #[track_caller]
/* FP:lib.rs-1240 */     pub fn create_bug(self, bug: impl Diagnostic<'a, BugAbort>) -> Diag<'a, BugAbort> {
/* FP:lib.rs-1241 */         bug.into_diag(self, Bug)
/* FP:lib.rs-1242 */     }
/* FP:lib.rs-1243 */ 
/* FP:lib.rs-1244 */     #[track_caller]
/* FP:lib.rs-1245 */     pub fn emit_bug(self, bug: impl Diagnostic<'a, BugAbort>) -> ! {
/* FP:lib.rs-1246 */         self.create_bug(bug).emit()
/* FP:lib.rs-1247 */     }
/* FP:lib.rs-1248 */ 
/* FP:lib.rs-1249 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1250 */     #[track_caller]
/* FP:lib.rs-1251 */     pub fn struct_fatal(self, msg: impl Into<DiagMessage>) -> Diag<'a, FatalAbort> {
/* FP:lib.rs-1252 */         Diag::new(self, Fatal, msg)
/* FP:lib.rs-1253 */     }
/* FP:lib.rs-1254 */ 
/* FP:lib.rs-1255 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1256 */     #[track_caller]
/* FP:lib.rs-1257 */     pub fn fatal(self, msg: impl Into<DiagMessage>) -> ! {
/* FP:lib.rs-1258 */         self.struct_fatal(msg).emit()
/* FP:lib.rs-1259 */     }
/* FP:lib.rs-1260 */ 
/* FP:lib.rs-1261 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1262 */     #[track_caller]
/* FP:lib.rs-1263 */     pub fn struct_span_fatal(
/* FP:lib.rs-1264 */         self,
/* FP:lib.rs-1265 */         span: impl Into<MultiSpan>,
/* FP:lib.rs-1266 */         msg: impl Into<DiagMessage>,
/* FP:lib.rs-1267 */     ) -> Diag<'a, FatalAbort> {
/* FP:lib.rs-1268 */         self.struct_fatal(msg).with_span(span)
/* FP:lib.rs-1269 */     }
/* FP:lib.rs-1270 */ 
/* FP:lib.rs-1271 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1272 */     #[track_caller]
/* FP:lib.rs-1273 */     pub fn span_fatal(self, span: impl Into<MultiSpan>, msg: impl Into<DiagMessage>) -> ! {
/* FP:lib.rs-1274 */         self.struct_span_fatal(span, msg).emit()
/* FP:lib.rs-1275 */     }
/* FP:lib.rs-1276 */ 
/* FP:lib.rs-1277 */     #[track_caller]
/* FP:lib.rs-1278 */     pub fn create_fatal(self, fatal: impl Diagnostic<'a, FatalAbort>) -> Diag<'a, FatalAbort> {
/* FP:lib.rs-1279 */         fatal.into_diag(self, Fatal)
/* FP:lib.rs-1280 */     }
/* FP:lib.rs-1281 */ 
/* FP:lib.rs-1282 */     #[track_caller]
/* FP:lib.rs-1283 */     pub fn emit_fatal(self, fatal: impl Diagnostic<'a, FatalAbort>) -> ! {
/* FP:lib.rs-1284 */         self.create_fatal(fatal).emit()
/* FP:lib.rs-1285 */     }
/* FP:lib.rs-1286 */ 
/* FP:lib.rs-1287 */     #[track_caller]
/* FP:lib.rs-1288 */     pub fn create_almost_fatal(
/* FP:lib.rs-1289 */         self,
/* FP:lib.rs-1290 */         fatal: impl Diagnostic<'a, FatalError>,
/* FP:lib.rs-1291 */     ) -> Diag<'a, FatalError> {
/* FP:lib.rs-1292 */         fatal.into_diag(self, Fatal)
/* FP:lib.rs-1293 */     }
/* FP:lib.rs-1294 */ 
/* FP:lib.rs-1295 */     #[track_caller]
/* FP:lib.rs-1296 */     pub fn emit_almost_fatal(self, fatal: impl Diagnostic<'a, FatalError>) -> FatalError {
/* FP:lib.rs-1297 */         self.create_almost_fatal(fatal).emit()
/* FP:lib.rs-1298 */     }
/* FP:lib.rs-1299 */ 
/* FP:lib.rs-1300 */     // FIXME: This method should be removed (every error should have an associated error code).
/* FP:lib.rs-1301 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1302 */     #[track_caller]
/* FP:lib.rs-1303 */     pub fn struct_err(self, msg: impl Into<DiagMessage>) -> Diag<'a> {
/* FP:lib.rs-1304 */         Diag::new(self, Error, msg)
/* FP:lib.rs-1305 */     }
/* FP:lib.rs-1306 */ 
/* FP:lib.rs-1307 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1308 */     #[track_caller]
/* FP:lib.rs-1309 */     pub fn err(self, msg: impl Into<DiagMessage>) -> ErrorGuaranteed {
/* FP:lib.rs-1310 */         self.struct_err(msg).emit()
/* FP:lib.rs-1311 */     }
/* FP:lib.rs-1312 */ 
/* FP:lib.rs-1313 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1314 */     #[track_caller]
/* FP:lib.rs-1315 */     pub fn struct_span_err(
/* FP:lib.rs-1316 */         self,
/* FP:lib.rs-1317 */         span: impl Into<MultiSpan>,
/* FP:lib.rs-1318 */         msg: impl Into<DiagMessage>,
/* FP:lib.rs-1319 */     ) -> Diag<'a> {
/* FP:lib.rs-1320 */         self.struct_err(msg).with_span(span)
/* FP:lib.rs-1321 */     }
/* FP:lib.rs-1322 */ 
/* FP:lib.rs-1323 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1324 */     #[track_caller]
/* FP:lib.rs-1325 */     pub fn span_err(
/* FP:lib.rs-1326 */         self,
/* FP:lib.rs-1327 */         span: impl Into<MultiSpan>,
/* FP:lib.rs-1328 */         msg: impl Into<DiagMessage>,
/* FP:lib.rs-1329 */     ) -> ErrorGuaranteed {
/* FP:lib.rs-1330 */         self.struct_span_err(span, msg).emit()
/* FP:lib.rs-1331 */     }
/* FP:lib.rs-1332 */ 
/* FP:lib.rs-1333 */     #[track_caller]
/* FP:lib.rs-1334 */     pub fn create_err(self, err: impl Diagnostic<'a>) -> Diag<'a> {
/* FP:lib.rs-1335 */         err.into_diag(self, Error)
/* FP:lib.rs-1336 */     }
/* FP:lib.rs-1337 */ 
/* FP:lib.rs-1338 */     #[track_caller]
/* FP:lib.rs-1339 */     pub fn emit_err(self, err: impl Diagnostic<'a>) -> ErrorGuaranteed {
/* FP:lib.rs-1340 */         self.create_err(err).emit()
/* FP:lib.rs-1341 */     }
/* FP:lib.rs-1342 */ 
/* FP:lib.rs-1343 */     /// Ensures that an error is printed. See `Level::DelayedBug`.
/* FP:lib.rs-1344 */     //
/* FP:lib.rs-1345 */     // No `#[rustc_lint_diagnostics]` and no `impl Into<DiagMessage>` because bug messages aren't
/* FP:lib.rs-1346 */     // user-facing.
/* FP:lib.rs-1347 */     #[track_caller]
/* FP:lib.rs-1348 */     pub fn delayed_bug(self, msg: impl Into<Cow<'static, str>>) -> ErrorGuaranteed {
/* FP:lib.rs-1349 */         Diag::<ErrorGuaranteed>::new(self, DelayedBug, msg.into()).emit()
/* FP:lib.rs-1350 */     }
/* FP:lib.rs-1351 */ 
/* FP:lib.rs-1352 */     /// Ensures that an error is printed. See [`Level::DelayedBug`].
/* FP:lib.rs-1353 */     ///
/* FP:lib.rs-1354 */     /// Note: this function used to be called `delay_span_bug`. It was renamed
/* FP:lib.rs-1355 */     /// to match similar functions like `span_err`, `span_warn`, etc.
/* FP:lib.rs-1356 */     //
/* FP:lib.rs-1357 */     // No `#[rustc_lint_diagnostics]` and no `impl Into<DiagMessage>` because bug messages aren't
/* FP:lib.rs-1358 */     // user-facing.
/* FP:lib.rs-1359 */     #[track_caller]
/* FP:lib.rs-1360 */     pub fn span_delayed_bug(
/* FP:lib.rs-1361 */         self,
/* FP:lib.rs-1362 */         sp: impl Into<MultiSpan>,
/* FP:lib.rs-1363 */         msg: impl Into<Cow<'static, str>>,
/* FP:lib.rs-1364 */     ) -> ErrorGuaranteed {
/* FP:lib.rs-1365 */         Diag::<ErrorGuaranteed>::new(self, DelayedBug, msg.into()).with_span(sp).emit()
/* FP:lib.rs-1366 */     }
/* FP:lib.rs-1367 */ 
/* FP:lib.rs-1368 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1369 */     #[track_caller]
/* FP:lib.rs-1370 */     pub fn struct_warn(self, msg: impl Into<DiagMessage>) -> Diag<'a, ()> {
/* FP:lib.rs-1371 */         Diag::new(self, Warning, msg)
/* FP:lib.rs-1372 */     }
/* FP:lib.rs-1373 */ 
/* FP:lib.rs-1374 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1375 */     #[track_caller]
/* FP:lib.rs-1376 */     pub fn warn(self, msg: impl Into<DiagMessage>) {
/* FP:lib.rs-1377 */         self.struct_warn(msg).emit()
/* FP:lib.rs-1378 */     }
/* FP:lib.rs-1379 */ 
/* FP:lib.rs-1380 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1381 */     #[track_caller]
/* FP:lib.rs-1382 */     pub fn struct_span_warn(
/* FP:lib.rs-1383 */         self,
/* FP:lib.rs-1384 */         span: impl Into<MultiSpan>,
/* FP:lib.rs-1385 */         msg: impl Into<DiagMessage>,
/* FP:lib.rs-1386 */     ) -> Diag<'a, ()> {
/* FP:lib.rs-1387 */         self.struct_warn(msg).with_span(span)
/* FP:lib.rs-1388 */     }
/* FP:lib.rs-1389 */ 
/* FP:lib.rs-1390 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1391 */     #[track_caller]
/* FP:lib.rs-1392 */     pub fn span_warn(self, span: impl Into<MultiSpan>, msg: impl Into<DiagMessage>) {
/* FP:lib.rs-1393 */         self.struct_span_warn(span, msg).emit()
/* FP:lib.rs-1394 */     }
/* FP:lib.rs-1395 */ 
/* FP:lib.rs-1396 */     #[track_caller]
/* FP:lib.rs-1397 */     pub fn create_warn(self, warning: impl Diagnostic<'a, ()>) -> Diag<'a, ()> {
/* FP:lib.rs-1398 */         warning.into_diag(self, Warning)
/* FP:lib.rs-1399 */     }
/* FP:lib.rs-1400 */ 
/* FP:lib.rs-1401 */     #[track_caller]
/* FP:lib.rs-1402 */     pub fn emit_warn(self, warning: impl Diagnostic<'a, ()>) {
/* FP:lib.rs-1403 */         self.create_warn(warning).emit()
/* FP:lib.rs-1404 */     }
/* FP:lib.rs-1405 */ 
/* FP:lib.rs-1406 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1407 */     #[track_caller]
/* FP:lib.rs-1408 */     pub fn struct_note(self, msg: impl Into<DiagMessage>) -> Diag<'a, ()> {
/* FP:lib.rs-1409 */         Diag::new(self, Note, msg)
/* FP:lib.rs-1410 */     }
/* FP:lib.rs-1411 */ 
/* FP:lib.rs-1412 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1413 */     #[track_caller]
/* FP:lib.rs-1414 */     pub fn note(&self, msg: impl Into<DiagMessage>) {
/* FP:lib.rs-1415 */         self.struct_note(msg).emit()
/* FP:lib.rs-1416 */     }
/* FP:lib.rs-1417 */ 
/* FP:lib.rs-1418 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1419 */     #[track_caller]
/* FP:lib.rs-1420 */     pub fn struct_span_note(
/* FP:lib.rs-1421 */         self,
/* FP:lib.rs-1422 */         span: impl Into<MultiSpan>,
/* FP:lib.rs-1423 */         msg: impl Into<DiagMessage>,
/* FP:lib.rs-1424 */     ) -> Diag<'a, ()> {
/* FP:lib.rs-1425 */         self.struct_note(msg).with_span(span)
/* FP:lib.rs-1426 */     }
/* FP:lib.rs-1427 */ 
/* FP:lib.rs-1428 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1429 */     #[track_caller]
/* FP:lib.rs-1430 */     pub fn span_note(self, span: impl Into<MultiSpan>, msg: impl Into<DiagMessage>) {
/* FP:lib.rs-1431 */         self.struct_span_note(span, msg).emit()
/* FP:lib.rs-1432 */     }
/* FP:lib.rs-1433 */ 
/* FP:lib.rs-1434 */     #[track_caller]
/* FP:lib.rs-1435 */     pub fn create_note(self, note: impl Diagnostic<'a, ()>) -> Diag<'a, ()> {
/* FP:lib.rs-1436 */         note.into_diag(self, Note)
/* FP:lib.rs-1437 */     }
/* FP:lib.rs-1438 */ 
/* FP:lib.rs-1439 */     #[track_caller]
/* FP:lib.rs-1440 */     pub fn emit_note(self, note: impl Diagnostic<'a, ()>) {
/* FP:lib.rs-1441 */         self.create_note(note).emit()
/* FP:lib.rs-1442 */     }
/* FP:lib.rs-1443 */ 
/* FP:lib.rs-1444 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1445 */     #[track_caller]
/* FP:lib.rs-1446 */     pub fn struct_help(self, msg: impl Into<DiagMessage>) -> Diag<'a, ()> {
/* FP:lib.rs-1447 */         Diag::new(self, Help, msg)
/* FP:lib.rs-1448 */     }
/* FP:lib.rs-1449 */ 
/* FP:lib.rs-1450 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1451 */     #[track_caller]
/* FP:lib.rs-1452 */     pub fn struct_failure_note(self, msg: impl Into<DiagMessage>) -> Diag<'a, ()> {
/* FP:lib.rs-1453 */         Diag::new(self, FailureNote, msg)
/* FP:lib.rs-1454 */     }
/* FP:lib.rs-1455 */ 
/* FP:lib.rs-1456 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1457 */     #[track_caller]
/* FP:lib.rs-1458 */     pub fn struct_allow(self, msg: impl Into<DiagMessage>) -> Diag<'a, ()> {
/* FP:lib.rs-1459 */         Diag::new(self, Allow, msg)
/* FP:lib.rs-1460 */     }
/* FP:lib.rs-1461 */ 
/* FP:lib.rs-1462 */     #[rustc_lint_diagnostics]
/* FP:lib.rs-1463 */     #[track_caller]
/* FP:lib.rs-1464 */     pub fn struct_expect(self, msg: impl Into<DiagMessage>, id: LintExpectationId) -> Diag<'a, ()> {
/* FP:lib.rs-1465 */         Diag::new(self, Expect, msg).with_lint_id(id)
/* FP:lib.rs-1466 */     }
/* FP:lib.rs-1467 */ }
/* FP:lib.rs-1468 */ 
/* FP:lib.rs-1469 */ // Note: we prefer implementing operations on `DiagCtxt`, rather than
/* FP:lib.rs-1470 */ // `DiagCtxtInner`, whenever possible. This minimizes functions where
/* FP:lib.rs-1471 */ // `DiagCtxt::foo()` just borrows `inner` and forwards a call to
/* FP:lib.rs-1472 */ // `DiagCtxtInner::foo`.
/* FP:lib.rs-1473 */ impl DiagCtxtInner {
/* FP:lib.rs-1474 */     fn new(emitter: Box<DynEmitter>) -> Self {
/* FP:lib.rs-1475 */         Self {
/* FP:lib.rs-1476 */             flags: DiagCtxtFlags { can_emit_warnings: true, ..Default::default() },
/* FP:lib.rs-1477 */             registry: Registry::new(&[]),
/* FP:lib.rs-1478 */             err_guars: Vec::new(),
/* FP:lib.rs-1479 */             lint_err_guars: Vec::new(),
/* FP:lib.rs-1480 */             delayed_bugs: Vec::new(),
/* FP:lib.rs-1481 */             deduplicated_err_count: 0,
/* FP:lib.rs-1482 */             deduplicated_warn_count: 0,
/* FP:lib.rs-1483 */             emitter,
/* FP:lib.rs-1484 */             must_produce_diag: None,
/* FP:lib.rs-1485 */             has_printed: false,
/* FP:lib.rs-1486 */             suppressed_expected_diag: false,
/* FP:lib.rs-1487 */             taught_diagnostics: Default::default(),
/* FP:lib.rs-1488 */             emitted_diagnostic_codes: Default::default(),
/* FP:lib.rs-1489 */             emitted_diagnostics: Default::default(),
/* FP:lib.rs-1490 */             stashed_diagnostics: Default::default(),
/* FP:lib.rs-1491 */             future_breakage_diagnostics: Vec::new(),
/* FP:lib.rs-1492 */             fulfilled_expectations: Default::default(),
/* FP:lib.rs-1493 */             ice_file: None,
/* FP:lib.rs-1494 */         }
/* FP:lib.rs-1495 */     }
/* FP:lib.rs-1496 */ 
/* FP:lib.rs-1497 */     /// Emit all stashed diagnostics.
/* FP:lib.rs-1498 */     fn emit_stashed_diagnostics(&mut self) -> Option<ErrorGuaranteed> {
/* FP:lib.rs-1499 */         let mut guar = None;
/* FP:lib.rs-1500 */         let has_errors = !self.err_guars.is_empty();
/* FP:lib.rs-1501 */         for (_, stashed_diagnostics) in std::mem::take(&mut self.stashed_diagnostics).into_iter() {
/* FP:lib.rs-1502 */             for (_, (diag, _guar)) in stashed_diagnostics {
/* FP:lib.rs-1503 */                 if !diag.is_error() {
/* FP:lib.rs-1504 */                     // Unless they're forced, don't flush stashed warnings when
/* FP:lib.rs-1505 */                     // there are errors, to avoid causing warning overload. The
/* FP:lib.rs-1506 */                     // stash would've been stolen already if it were important.
/* FP:lib.rs-1507 */                     if !diag.is_force_warn() && has_errors {
/* FP:lib.rs-1508 */                         continue;
/* FP:lib.rs-1509 */                     }
/* FP:lib.rs-1510 */                 }
/* FP:lib.rs-1511 */                 guar = guar.or(self.emit_diagnostic(diag, None));
/* FP:lib.rs-1512 */             }
/* FP:lib.rs-1513 */         }
/* FP:lib.rs-1514 */         guar
/* FP:lib.rs-1515 */     }
/* FP:lib.rs-1516 */ 
/* FP:lib.rs-1517 */     // Return value is only `Some` if the level is `Error` or `DelayedBug`.
/* FP:lib.rs-1518 */     fn emit_diagnostic(
/* FP:lib.rs-1519 */         &mut self,
/* FP:lib.rs-1520 */         mut diagnostic: DiagInner,
/* FP:lib.rs-1521 */         taint: Option<&Cell<Option<ErrorGuaranteed>>>,
/* FP:lib.rs-1522 */     ) -> Option<ErrorGuaranteed> {
/* FP:lib.rs-1523 */         if diagnostic.has_future_breakage() {
/* FP:lib.rs-1524 */             // Future breakages aren't emitted if they're `Level::Allow` or
/* FP:lib.rs-1525 */             // `Level::Expect`, but they still need to be constructed and
/* FP:lib.rs-1526 */             // stashed below, so they'll trigger the must_produce_diag check.
/* FP:lib.rs-1527 */             assert_matches!(diagnostic.level, Error | ForceWarning | Warning | Allow | Expect);
/* FP:lib.rs-1528 */             self.future_breakage_diagnostics.push(diagnostic.clone());
/* FP:lib.rs-1529 */         }
/* FP:lib.rs-1530 */ 
/* FP:lib.rs-1531 */         // We call TRACK_DIAGNOSTIC with an empty closure for the cases that
/* FP:lib.rs-1532 */         // return early *and* have some kind of side-effect, except where
/* FP:lib.rs-1533 */         // noted.
/* FP:lib.rs-1534 */         match diagnostic.level {
/* FP:lib.rs-1535 */             Bug => {}
/* FP:lib.rs-1536 */             Fatal | Error => {
/* FP:lib.rs-1537 */                 if self.treat_next_err_as_bug() {
/* FP:lib.rs-1538 */                     // `Fatal` and `Error` can be promoted to `Bug`.
/* FP:lib.rs-1539 */                     diagnostic.level = Bug;
/* FP:lib.rs-1540 */                 }
/* FP:lib.rs-1541 */             }
/* FP:lib.rs-1542 */             DelayedBug => {
/* FP:lib.rs-1543 */                 // Note that because we check these conditions first,
/* FP:lib.rs-1544 */                 // `-Zeagerly-emit-delayed-bugs` and `-Ztreat-err-as-bug`
/* FP:lib.rs-1545 */                 // continue to work even after we've issued an error and
/* FP:lib.rs-1546 */                 // stopped recording new delayed bugs.
/* FP:lib.rs-1547 */                 if self.flags.eagerly_emit_delayed_bugs {
/* FP:lib.rs-1548 */                     // `DelayedBug` can be promoted to `Error` or `Bug`.
/* FP:lib.rs-1549 */                     if self.treat_next_err_as_bug() {
/* FP:lib.rs-1550 */                         diagnostic.level = Bug;
/* FP:lib.rs-1551 */                     } else {
/* FP:lib.rs-1552 */                         diagnostic.level = Error;
/* FP:lib.rs-1553 */                     }
/* FP:lib.rs-1554 */                 } else {
/* FP:lib.rs-1555 */                     // If we have already emitted at least one error, we don't need
/* FP:lib.rs-1556 */                     // to record the delayed bug, because it'll never be used.
/* FP:lib.rs-1557 */                     return if let Some(guar) = self.has_errors() {
/* FP:lib.rs-1558 */                         Some(guar)
/* FP:lib.rs-1559 */                     } else {
/* FP:lib.rs-1560 */                         // No `TRACK_DIAGNOSTIC` call is needed, because the
/* FP:lib.rs-1561 */                         // incremental session is deleted if there is a delayed
/* FP:lib.rs-1562 */                         // bug. This also saves us from cloning the diagnostic.
/* FP:lib.rs-1563 */                         let backtrace = std::backtrace::Backtrace::capture();
/* FP:lib.rs-1564 */                         // This `unchecked_error_guaranteed` is valid. It is where the
/* FP:lib.rs-1565 */                         // `ErrorGuaranteed` for delayed bugs originates. See
/* FP:lib.rs-1566 */                         // `DiagCtxtInner::drop`.
/* FP:lib.rs-1567 */                         #[allow(deprecated)]
/* FP:lib.rs-1568 */                         let guar = ErrorGuaranteed::unchecked_error_guaranteed();
/* FP:lib.rs-1569 */                         self.delayed_bugs
/* FP:lib.rs-1570 */                             .push((DelayedDiagInner::with_backtrace(diagnostic, backtrace), guar));
/* FP:lib.rs-1571 */                         Some(guar)
/* FP:lib.rs-1572 */                     };
/* FP:lib.rs-1573 */                 }
/* FP:lib.rs-1574 */             }
/* FP:lib.rs-1575 */             ForceWarning if diagnostic.lint_id.is_none() => {} // `ForceWarning(Some(...))` is below, with `Expect`
/* FP:lib.rs-1576 */             Warning => {
/* FP:lib.rs-1577 */                 if !self.flags.can_emit_warnings {
/* FP:lib.rs-1578 */                     // We are not emitting warnings.
/* FP:lib.rs-1579 */                     if diagnostic.has_future_breakage() {
/* FP:lib.rs-1580 */                         // The side-effect is at the top of this method.
/* FP:lib.rs-1581 */                         TRACK_DIAGNOSTIC(diagnostic, &mut |_| None);
/* FP:lib.rs-1582 */                     }
/* FP:lib.rs-1583 */                     return None;
/* FP:lib.rs-1584 */                 }
/* FP:lib.rs-1585 */             }
/* FP:lib.rs-1586 */             Note | Help | FailureNote => {}
/* FP:lib.rs-1587 */             OnceNote | OnceHelp => panic!("bad level: {:?}", diagnostic.level),
/* FP:lib.rs-1588 */             Allow => {
/* FP:lib.rs-1589 */                 // Nothing emitted for allowed lints.
/* FP:lib.rs-1590 */                 if diagnostic.has_future_breakage() {
/* FP:lib.rs-1591 */                     // The side-effect is at the top of this method.
/* FP:lib.rs-1592 */                     TRACK_DIAGNOSTIC(diagnostic, &mut |_| None);
/* FP:lib.rs-1593 */                     self.suppressed_expected_diag = true;
/* FP:lib.rs-1594 */                 }
/* FP:lib.rs-1595 */                 return None;
/* FP:lib.rs-1596 */             }
/* FP:lib.rs-1597 */             Expect | ForceWarning => {
/* FP:lib.rs-1598 */                 self.fulfilled_expectations.insert(diagnostic.lint_id.unwrap());
/* FP:lib.rs-1599 */                 if let Expect = diagnostic.level {
/* FP:lib.rs-1600 */                     // Nothing emitted here for expected lints.
/* FP:lib.rs-1601 */                     TRACK_DIAGNOSTIC(diagnostic, &mut |_| None);
/* FP:lib.rs-1602 */                     self.suppressed_expected_diag = true;
/* FP:lib.rs-1603 */                     return None;
/* FP:lib.rs-1604 */                 }
/* FP:lib.rs-1605 */             }
/* FP:lib.rs-1606 */         }
/* FP:lib.rs-1607 */ 
/* FP:lib.rs-1608 */         TRACK_DIAGNOSTIC(diagnostic, &mut |mut diagnostic| {
/* FP:lib.rs-1609 */             if let Some(code) = diagnostic.code {
/* FP:lib.rs-1610 */                 self.emitted_diagnostic_codes.insert(code);
/* FP:lib.rs-1611 */             }
/* FP:lib.rs-1612 */ 
/* FP:lib.rs-1613 */             let already_emitted = {
/* FP:lib.rs-1614 */                 let mut hasher = StableHasher::new();
/* FP:lib.rs-1615 */                 diagnostic.hash(&mut hasher);
/* FP:lib.rs-1616 */                 let diagnostic_hash = hasher.finish();
/* FP:lib.rs-1617 */                 !self.emitted_diagnostics.insert(diagnostic_hash)
/* FP:lib.rs-1618 */             };
/* FP:lib.rs-1619 */ 
/* FP:lib.rs-1620 */             let is_error = diagnostic.is_error();
/* FP:lib.rs-1621 */             let is_lint = diagnostic.is_lint.is_some();
/* FP:lib.rs-1622 */ 
/* FP:lib.rs-1623 */             // Only emit the diagnostic if we've been asked to deduplicate or
/* FP:lib.rs-1624 */             // haven't already emitted an equivalent diagnostic.
/* FP:lib.rs-1625 */             if !(self.flags.deduplicate_diagnostics && already_emitted) {
/* FP:lib.rs-1626 */                 debug!(?diagnostic);
/* FP:lib.rs-1627 */                 debug!(?self.emitted_diagnostics);
/* FP:lib.rs-1628 */ 
/* FP:lib.rs-1629 */                 let not_yet_emitted = |sub: &mut Subdiag| {
/* FP:lib.rs-1630 */                     debug!(?sub);
/* FP:lib.rs-1631 */                     if sub.level != OnceNote && sub.level != OnceHelp {
/* FP:lib.rs-1632 */                         return true;
/* FP:lib.rs-1633 */                     }
/* FP:lib.rs-1634 */                     let mut hasher = StableHasher::new();
/* FP:lib.rs-1635 */                     sub.hash(&mut hasher);
/* FP:lib.rs-1636 */                     let diagnostic_hash = hasher.finish();
/* FP:lib.rs-1637 */                     debug!(?diagnostic_hash);
/* FP:lib.rs-1638 */                     self.emitted_diagnostics.insert(diagnostic_hash)
/* FP:lib.rs-1639 */                 };
/* FP:lib.rs-1640 */                 diagnostic.children.retain_mut(not_yet_emitted);
/* FP:lib.rs-1641 */                 if already_emitted {
/* FP:lib.rs-1642 */                     let msg = "duplicate diagnostic emitted due to `-Z deduplicate-diagnostics=no`";
/* FP:lib.rs-1643 */                     diagnostic.sub(Note, msg, MultiSpan::new());
/* FP:lib.rs-1644 */                 }
/* FP:lib.rs-1645 */ 
/* FP:lib.rs-1646 */                 if is_error {
/* FP:lib.rs-1647 */                     self.deduplicated_err_count += 1;
/* FP:lib.rs-1648 */                 } else if matches!(diagnostic.level, ForceWarning | Warning) {
/* FP:lib.rs-1649 */                     self.deduplicated_warn_count += 1;
/* FP:lib.rs-1650 */                 }
/* FP:lib.rs-1651 */                 self.has_printed = true;
/* FP:lib.rs-1652 */ 
/* FP:lib.rs-1653 */                 self.emitter.emit_diagnostic(diagnostic, &self.registry);
/* FP:lib.rs-1654 */             }
/* FP:lib.rs-1655 */ 
/* FP:lib.rs-1656 */             if is_error {
/* FP:lib.rs-1657 */                 // If we have any delayed bugs recorded, we can discard them
/* FP:lib.rs-1658 */                 // because they won't be used. (This should only occur if there
/* FP:lib.rs-1659 */                 // have been no errors previously emitted, because we don't add
/* FP:lib.rs-1660 */                 // new delayed bugs once the first error is emitted.)
/* FP:lib.rs-1661 */                 if !self.delayed_bugs.is_empty() {
/* FP:lib.rs-1662 */                     assert_eq!(self.lint_err_guars.len() + self.err_guars.len(), 0);
/* FP:lib.rs-1663 */                     self.delayed_bugs.clear();
/* FP:lib.rs-1664 */                     self.delayed_bugs.shrink_to_fit();
/* FP:lib.rs-1665 */                 }
/* FP:lib.rs-1666 */ 
/* FP:lib.rs-1667 */                 // This `unchecked_error_guaranteed` is valid. It is where the
/* FP:lib.rs-1668 */                 // `ErrorGuaranteed` for errors and lint errors originates.
/* FP:lib.rs-1669 */                 #[allow(deprecated)]
/* FP:lib.rs-1670 */                 let guar = ErrorGuaranteed::unchecked_error_guaranteed();
/* FP:lib.rs-1671 */                 if is_lint {
/* FP:lib.rs-1672 */                     self.lint_err_guars.push(guar);
/* FP:lib.rs-1673 */                 } else {
/* FP:lib.rs-1674 */                     if let Some(taint) = taint {
/* FP:lib.rs-1675 */                         taint.set(Some(guar));
/* FP:lib.rs-1676 */                     }
/* FP:lib.rs-1677 */                     self.err_guars.push(guar);
/* FP:lib.rs-1678 */                 }
/* FP:lib.rs-1679 */                 self.panic_if_treat_err_as_bug();
/* FP:lib.rs-1680 */                 Some(guar)
/* FP:lib.rs-1681 */             } else {
/* FP:lib.rs-1682 */                 None
/* FP:lib.rs-1683 */             }
/* FP:lib.rs-1684 */         })
/* FP:lib.rs-1685 */     }
/* FP:lib.rs-1686 */ 
/* FP:lib.rs-1687 */     fn treat_err_as_bug(&self) -> bool {
/* FP:lib.rs-1688 */         self.flags
/* FP:lib.rs-1689 */             .treat_err_as_bug
/* FP:lib.rs-1690 */             .is_some_and(|c| self.err_guars.len() + self.lint_err_guars.len() >= c.get())
/* FP:lib.rs-1691 */     }
/* FP:lib.rs-1692 */ 
/* FP:lib.rs-1693 */     // Use this one before incrementing `err_count`.
/* FP:lib.rs-1694 */     fn treat_next_err_as_bug(&self) -> bool {
/* FP:lib.rs-1695 */         self.flags
/* FP:lib.rs-1696 */             .treat_err_as_bug
/* FP:lib.rs-1697 */             .is_some_and(|c| self.err_guars.len() + self.lint_err_guars.len() + 1 >= c.get())
/* FP:lib.rs-1698 */     }
/* FP:lib.rs-1699 */ 
/* FP:lib.rs-1700 */     fn has_errors_excluding_lint_errors(&self) -> Option<ErrorGuaranteed> {
/* FP:lib.rs-1701 */         self.err_guars.get(0).copied().or_else(|| {
/* FP:lib.rs-1702 */             if let Some((_diag, guar)) = self
/* FP:lib.rs-1703 */                 .stashed_diagnostics
/* FP:lib.rs-1704 */                 .values()
/* FP:lib.rs-1705 */                 .flat_map(|stashed_diagnostics| stashed_diagnostics.values())
/* FP:lib.rs-1706 */                 .find(|(diag, guar)| guar.is_some() && diag.is_lint.is_none())
/* FP:lib.rs-1707 */             {
/* FP:lib.rs-1708 */                 *guar
/* FP:lib.rs-1709 */             } else {
/* FP:lib.rs-1710 */                 None
/* FP:lib.rs-1711 */             }
/* FP:lib.rs-1712 */         })
/* FP:lib.rs-1713 */     }
/* FP:lib.rs-1714 */ 
/* FP:lib.rs-1715 */     fn has_errors(&self) -> Option<ErrorGuaranteed> {
/* FP:lib.rs-1716 */         self.err_guars.get(0).copied().or_else(|| self.lint_err_guars.get(0).copied()).or_else(
/* FP:lib.rs-1717 */             || {
/* FP:lib.rs-1718 */                 self.stashed_diagnostics.values().find_map(|stashed_diagnostics| {
/* FP:lib.rs-1719 */                     stashed_diagnostics.values().find_map(|(_, guar)| *guar)
/* FP:lib.rs-1720 */                 })
/* FP:lib.rs-1721 */             },
/* FP:lib.rs-1722 */         )
/* FP:lib.rs-1723 */     }
/* FP:lib.rs-1724 */ 
/* FP:lib.rs-1725 */     fn has_errors_or_delayed_bugs(&self) -> Option<ErrorGuaranteed> {
/* FP:lib.rs-1726 */         self.has_errors().or_else(|| self.delayed_bugs.get(0).map(|(_, guar)| guar).copied())
/* FP:lib.rs-1727 */     }
/* FP:lib.rs-1728 */ 
/* FP:lib.rs-1729 */     /// Translate `message` eagerly with `args` to `SubdiagMessage::Eager`.
/* FP:lib.rs-1730 */     fn eagerly_translate<'a>(
/* FP:lib.rs-1731 */         &self,
/* FP:lib.rs-1732 */         message: DiagMessage,
/* FP:lib.rs-1733 */         args: impl Iterator<Item = DiagArg<'a>>,
/* FP:lib.rs-1734 */     ) -> SubdiagMessage {
/* FP:lib.rs-1735 */         SubdiagMessage::Translated(Cow::from(self.eagerly_translate_to_string(message, args)))
/* FP:lib.rs-1736 */     }
/* FP:lib.rs-1737 */ 
/* FP:lib.rs-1738 */     /// Translate `message` eagerly with `args` to `String`.
/* FP:lib.rs-1739 */     fn eagerly_translate_to_string<'a>(
/* FP:lib.rs-1740 */         &self,
/* FP:lib.rs-1741 */         message: DiagMessage,
/* FP:lib.rs-1742 */         args: impl Iterator<Item = DiagArg<'a>>,
/* FP:lib.rs-1743 */     ) -> String {
/* FP:lib.rs-1744 */         let args = crate::translation::to_fluent_args(args);
/* FP:lib.rs-1745 */         self.emitter
/* FP:lib.rs-1746 */             .translator()
/* FP:lib.rs-1747 */             .translate_message(&message, &args)
/* FP:lib.rs-1748 */             .map_err(Report::new)
/* FP:lib.rs-1749 */             .unwrap()
/* FP:lib.rs-1750 */             .to_string()
/* FP:lib.rs-1751 */     }
/* FP:lib.rs-1752 */ 
/* FP:lib.rs-1753 */     fn eagerly_translate_for_subdiag(
/* FP:lib.rs-1754 */         &self,
/* FP:lib.rs-1755 */         diag: &DiagInner,
/* FP:lib.rs-1756 */         msg: impl Into<SubdiagMessage>,
/* FP:lib.rs-1757 */     ) -> SubdiagMessage {
/* FP:lib.rs-1758 */         let msg = diag.subdiagnostic_message_to_diagnostic_message(msg);
/* FP:lib.rs-1759 */         self.eagerly_translate(msg, diag.args.iter())
/* FP:lib.rs-1760 */     }
/* FP:lib.rs-1761 */ 
/* FP:lib.rs-1762 */     fn flush_delayed(&mut self) {
/* FP:lib.rs-1763 */         // Stashed diagnostics must be emitted before delayed bugs are flushed.
/* FP:lib.rs-1764 */         // Otherwise, we might ICE prematurely when errors would have
/* FP:lib.rs-1765 */         // eventually happened.
/* FP:lib.rs-1766 */         assert!(self.stashed_diagnostics.is_empty());
/* FP:lib.rs-1767 */ 
/* FP:lib.rs-1768 */         if !self.err_guars.is_empty() {
/* FP:lib.rs-1769 */             // If an error happened already. We shouldn't expose delayed bugs.
/* FP:lib.rs-1770 */             return;
/* FP:lib.rs-1771 */         }
/* FP:lib.rs-1772 */ 
/* FP:lib.rs-1773 */         if self.delayed_bugs.is_empty() {
/* FP:lib.rs-1774 */             // Nothing to do.
/* FP:lib.rs-1775 */             return;
/* FP:lib.rs-1776 */         }
/* FP:lib.rs-1777 */ 
/* FP:lib.rs-1778 */         let bugs: Vec<_> =
/* FP:lib.rs-1779 */             std::mem::take(&mut self.delayed_bugs).into_iter().map(|(b, _)| b).collect();
/* FP:lib.rs-1780 */ 
/* FP:lib.rs-1781 */         let backtrace = std::env::var_os("RUST_BACKTRACE").as_deref() != Some(OsStr::new("0"));
/* FP:lib.rs-1782 */         let decorate = backtrace || self.ice_file.is_none();
/* FP:lib.rs-1783 */         let mut out = self
/* FP:lib.rs-1784 */             .ice_file
/* FP:lib.rs-1785 */             .as_ref()
/* FP:lib.rs-1786 */             .and_then(|file| std::fs::File::options().create(true).append(true).open(file).ok());
/* FP:lib.rs-1787 */ 
/* FP:lib.rs-1788 */         // Put the overall explanation before the `DelayedBug`s, to frame them
/* FP:lib.rs-1789 */         // better (e.g. separate warnings from them). Also, use notes, which
/* FP:lib.rs-1790 */         // don't count as errors, to avoid possibly triggering
/* FP:lib.rs-1791 */         // `-Ztreat-err-as-bug`, which we don't want.
/* FP:lib.rs-1792 */         let note1 = "no errors encountered even though delayed bugs were created";
/* FP:lib.rs-1793 */         let note2 = "those delayed bugs will now be shown as internal compiler errors";
/* FP:lib.rs-1794 */         self.emit_diagnostic(DiagInner::new(Note, note1), None);
/* FP:lib.rs-1795 */         self.emit_diagnostic(DiagInner::new(Note, note2), None);
/* FP:lib.rs-1796 */ 
/* FP:lib.rs-1797 */         for bug in bugs {
/* FP:lib.rs-1798 */             if let Some(out) = &mut out {
/* FP:lib.rs-1799 */                 _ = write!(
/* FP:lib.rs-1800 */                     out,
/* FP:lib.rs-1801 */                     "delayed bug: {}\n{}\n",
/* FP:lib.rs-1802 */                     bug.inner
/* FP:lib.rs-1803 */                         .messages
/* FP:lib.rs-1804 */                         .iter()
/* FP:lib.rs-1805 */                         .filter_map(|(msg, _)| msg.as_str())
/* FP:lib.rs-1806 */                         .collect::<String>(),
/* FP:lib.rs-1807 */                     &bug.note
/* FP:lib.rs-1808 */                 );
/* FP:lib.rs-1809 */             }
/* FP:lib.rs-1810 */ 
/* FP:lib.rs-1811 */             let mut bug = if decorate { bug.decorate(self) } else { bug.inner };
/* FP:lib.rs-1812 */ 
/* FP:lib.rs-1813 */             // "Undelay" the delayed bugs into plain bugs.
/* FP:lib.rs-1814 */             if bug.level != DelayedBug {
/* FP:lib.rs-1815 */                 // NOTE(eddyb) not panicking here because we're already producing
/* FP:lib.rs-1816 */                 // an ICE, and the more information the merrier.
/* FP:lib.rs-1817 */                 //
/* FP:lib.rs-1818 */                 // We are at the `DiagInner`/`DiagCtxtInner` level rather than
/* FP:lib.rs-1819 */                 // the usual `Diag`/`DiagCtxt` level, so we must augment `bug`
/* FP:lib.rs-1820 */                 // in a lower-level fashion.
/* FP:lib.rs-1821 */                 bug.arg("level", bug.level);
/* FP:lib.rs-1822 */                 let msg = crate::fluent_generated::errors_invalid_flushed_delayed_diagnostic_level;
/* FP:lib.rs-1823 */                 let msg = self.eagerly_translate_for_subdiag(&bug, msg); // after the `arg` call
/* FP:lib.rs-1824 */                 bug.sub(Note, msg, bug.span.primary_span().unwrap().into());
/* FP:lib.rs-1825 */             }
/* FP:lib.rs-1826 */             bug.level = Bug;
/* FP:lib.rs-1827 */ 
/* FP:lib.rs-1828 */             self.emit_diagnostic(bug, None);
/* FP:lib.rs-1829 */         }
/* FP:lib.rs-1830 */ 
/* FP:lib.rs-1831 */         // Panic with `DelayedBugPanic` to avoid "unexpected panic" messages.
/* FP:lib.rs-1832 */         panic::panic_any(DelayedBugPanic);
/* FP:lib.rs-1833 */     }
/* FP:lib.rs-1834 */ 
/* FP:lib.rs-1835 */     fn panic_if_treat_err_as_bug(&self) {
/* FP:lib.rs-1836 */         if self.treat_err_as_bug() {
/* FP:lib.rs-1837 */             let n = self.flags.treat_err_as_bug.map(|c| c.get()).unwrap();
/* FP:lib.rs-1838 */             assert_eq!(n, self.err_guars.len() + self.lint_err_guars.len());
/* FP:lib.rs-1839 */             if n == 1 {
/* FP:lib.rs-1840 */                 panic!("aborting due to `-Z treat-err-as-bug=1`");
/* FP:lib.rs-1841 */             } else {
/* FP:lib.rs-1842 */                 panic!("aborting after {n} errors due to `-Z treat-err-as-bug={n}`");
/* FP:lib.rs-1843 */             }
/* FP:lib.rs-1844 */         }
/* FP:lib.rs-1845 */     }
/* FP:lib.rs-1846 */ }
/* FP:lib.rs-1847 */ 
/* FP:lib.rs-1848 */ struct DelayedDiagInner {
/* FP:lib.rs-1849 */     inner: DiagInner,
/* FP:lib.rs-1850 */     note: Backtrace,
/* FP:lib.rs-1851 */ }
/* FP:lib.rs-1852 */ 
/* FP:lib.rs-1853 */ impl DelayedDiagInner {
/* FP:lib.rs-1854 */     fn with_backtrace(diagnostic: DiagInner, backtrace: Backtrace) -> Self {
/* FP:lib.rs-1855 */         DelayedDiagInner { inner: diagnostic, note: backtrace }
/* FP:lib.rs-1856 */     }
/* FP:lib.rs-1857 */ 
/* FP:lib.rs-1858 */     fn decorate(self, dcx: &DiagCtxtInner) -> DiagInner {
/* FP:lib.rs-1859 */         // We are at the `DiagInner`/`DiagCtxtInner` level rather than the
/* FP:lib.rs-1860 */         // usual `Diag`/`DiagCtxt` level, so we must construct `diag` in a
/* FP:lib.rs-1861 */         // lower-level fashion.
/* FP:lib.rs-1862 */         let mut diag = self.inner;
/* FP:lib.rs-1863 */         let msg = match self.note.status() {
/* FP:lib.rs-1864 */             BacktraceStatus::Captured => crate::fluent_generated::errors_delayed_at_with_newline,
/* FP:lib.rs-1865 */             // Avoid the needless newline when no backtrace has been captured,
/* FP:lib.rs-1866 */             // the display impl should just be a single line.
/* FP:lib.rs-1867 */             _ => crate::fluent_generated::errors_delayed_at_without_newline,
/* FP:lib.rs-1868 */         };
/* FP:lib.rs-1869 */         diag.arg("emitted_at", diag.emitted_at.clone());
/* FP:lib.rs-1870 */         diag.arg("note", self.note);
/* FP:lib.rs-1871 */         let msg = dcx.eagerly_translate_for_subdiag(&diag, msg); // after the `arg` calls
/* FP:lib.rs-1872 */         diag.sub(Note, msg, diag.span.primary_span().unwrap_or(DUMMY_SP).into());
/* FP:lib.rs-1873 */         diag
/* FP:lib.rs-1874 */     }
/* FP:lib.rs-1875 */ }
/* FP:lib.rs-1876 */ 
/* FP:lib.rs-1877 */ /// | Level        | is_error | EmissionGuarantee            | Top-level | Sub | Used in lints?
/* FP:lib.rs-1878 */ /// | -----        | -------- | -----------------            | --------- | --- | --------------
/* FP:lib.rs-1879 */ /// | Bug          | yes      | BugAbort                     | yes       | -   | -
/* FP:lib.rs-1880 */ /// | Fatal        | yes      | FatalAbort/FatalError[^star] | yes       | -   | -
/* FP:lib.rs-1881 */ /// | Error        | yes      | ErrorGuaranteed              | yes       | -   | yes
/* FP:lib.rs-1882 */ /// | DelayedBug   | yes      | ErrorGuaranteed              | yes       | -   | -
/* FP:lib.rs-1883 */ /// | ForceWarning | -        | ()                           | yes       | -   | lint-only
/* FP:lib.rs-1884 */ /// | Warning      | -        | ()                           | yes       | yes | yes
/* FP:lib.rs-1885 */ /// | Note         | -        | ()                           | rare      | yes | -
/* FP:lib.rs-1886 */ /// | OnceNote     | -        | ()                           | -         | yes | lint-only
/* FP:lib.rs-1887 */ /// | Help         | -        | ()                           | rare      | yes | -
/* FP:lib.rs-1888 */ /// | OnceHelp     | -        | ()                           | -         | yes | lint-only
/* FP:lib.rs-1889 */ /// | FailureNote  | -        | ()                           | rare      | -   | -
/* FP:lib.rs-1890 */ /// | Allow        | -        | ()                           | yes       | -   | lint-only
/* FP:lib.rs-1891 */ /// | Expect       | -        | ()                           | yes       | -   | lint-only
/* FP:lib.rs-1892 */ ///
/* FP:lib.rs-1893 */ /// [^star]: `FatalAbort` normally, `FatalError` in the non-aborting "almost fatal" case that is
/* FP:lib.rs-1894 */ ///     occasionally used.
/* FP:lib.rs-1895 */ ///
/* FP:lib.rs-1896 */ #[derive(Copy, PartialEq, Eq, Clone, Hash, Debug, Encodable, Decodable)]
/* FP:lib.rs-1897 */ pub enum Level {
/* FP:lib.rs-1898 */     /// For bugs in the compiler. Manifests as an ICE (internal compiler error) panic.
/* FP:lib.rs-1899 */     Bug,
/* FP:lib.rs-1900 */ 
/* FP:lib.rs-1901 */     /// An error that causes an immediate abort. Used for things like configuration errors,
/* FP:lib.rs-1902 */     /// internal overflows, some file operation errors.
/* FP:lib.rs-1903 */     Fatal,
/* FP:lib.rs-1904 */ 
/* FP:lib.rs-1905 */     /// An error in the code being compiled, which prevents compilation from finishing. This is the
/* FP:lib.rs-1906 */     /// most common case.
/* FP:lib.rs-1907 */     Error,
/* FP:lib.rs-1908 */ 
/* FP:lib.rs-1909 */     /// This is a strange one: lets you register an error without emitting it. If compilation ends
/* FP:lib.rs-1910 */     /// without any other errors occurring, this will be emitted as a bug. Otherwise, it will be
/* FP:lib.rs-1911 */     /// silently dropped. I.e. "expect other errors are emitted" semantics. Useful on code paths
/* FP:lib.rs-1912 */     /// that should only be reached when compiling erroneous code.
/* FP:lib.rs-1913 */     DelayedBug,
/* FP:lib.rs-1914 */ 
/* FP:lib.rs-1915 */     /// A `force-warn` lint warning about the code being compiled. Does not prevent compilation
/* FP:lib.rs-1916 */     /// from finishing.
/* FP:lib.rs-1917 */     ///
/* FP:lib.rs-1918 */     /// Requires a [`LintExpectationId`] for expected lint diagnostics. In all other cases this
/* FP:lib.rs-1919 */     /// should be `None`.
/* FP:lib.rs-1920 */     ForceWarning,
/* FP:lib.rs-1921 */ 
/* FP:lib.rs-1922 */     /// A warning about the code being compiled. Does not prevent compilation from finishing.
/* FP:lib.rs-1923 */     /// Will be skipped if `can_emit_warnings` is false.
/* FP:lib.rs-1924 */     Warning,
/* FP:lib.rs-1925 */ 
/* FP:lib.rs-1926 */     /// A message giving additional context.
/* FP:lib.rs-1927 */     Note,
/* FP:lib.rs-1928 */ 
/* FP:lib.rs-1929 */     /// A note that is only emitted once.
/* FP:lib.rs-1930 */     OnceNote,
/* FP:lib.rs-1931 */ 
/* FP:lib.rs-1932 */     /// A message suggesting how to fix something.
/* FP:lib.rs-1933 */     Help,
/* FP:lib.rs-1934 */ 
/* FP:lib.rs-1935 */     /// A help that is only emitted once.
/* FP:lib.rs-1936 */     OnceHelp,
/* FP:lib.rs-1937 */ 
/* FP:lib.rs-1938 */     /// Similar to `Note`, but used in cases where compilation has failed. When printed for human
/* FP:lib.rs-1939 */     /// consumption, it doesn't have any kind of `note:` label.
/* FP:lib.rs-1940 */     FailureNote,
/* FP:lib.rs-1941 */ 
/* FP:lib.rs-1942 */     /// Only used for lints.
/* FP:lib.rs-1943 */     Allow,
/* FP:lib.rs-1944 */ 
/* FP:lib.rs-1945 */     /// Only used for lints. Requires a [`LintExpectationId`] for silencing the lints.
/* FP:lib.rs-1946 */     Expect,
/* FP:lib.rs-1947 */ }
/* FP:lib.rs-1948 */ 
/* FP:lib.rs-1949 */ impl fmt::Display for Level {
/* FP:lib.rs-1950 */     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/* FP:lib.rs-1951 */         self.to_str().fmt(f)
/* FP:lib.rs-1952 */     }
/* FP:lib.rs-1953 */ }
/* FP:lib.rs-1954 */ 
/* FP:lib.rs-1955 */ impl Level {
/* FP:lib.rs-1956 */     fn color(self) -> ColorSpec {
/* FP:lib.rs-1957 */         let mut spec = ColorSpec::new();
/* FP:lib.rs-1958 */         match self {
/* FP:lib.rs-1959 */             Bug | Fatal | Error | DelayedBug => {
/* FP:lib.rs-1960 */                 spec.set_fg(Some(Color::Red)).set_intense(true);
/* FP:lib.rs-1961 */             }
/* FP:lib.rs-1962 */             ForceWarning | Warning => {
/* FP:lib.rs-1963 */                 spec.set_fg(Some(Color::Yellow)).set_intense(cfg!(windows));
/* FP:lib.rs-1964 */             }
/* FP:lib.rs-1965 */             Note | OnceNote => {
/* FP:lib.rs-1966 */                 spec.set_fg(Some(Color::Green)).set_intense(true);
/* FP:lib.rs-1967 */             }
/* FP:lib.rs-1968 */             Help | OnceHelp => {
/* FP:lib.rs-1969 */                 spec.set_fg(Some(Color::Cyan)).set_intense(true);
/* FP:lib.rs-1970 */             }
/* FP:lib.rs-1971 */             FailureNote => {}
/* FP:lib.rs-1972 */             Allow | Expect => unreachable!(),
/* FP:lib.rs-1973 */         }
/* FP:lib.rs-1974 */         spec
/* FP:lib.rs-1975 */     }
/* FP:lib.rs-1976 */ 
/* FP:lib.rs-1977 */     pub fn to_str(self) -> &'static str {
/* FP:lib.rs-1978 */         match self {
/* FP:lib.rs-1979 */             Bug | DelayedBug => "error: internal compiler error",
/* FP:lib.rs-1980 */             Fatal | Error => "error",
/* FP:lib.rs-1981 */             ForceWarning | Warning => "warning",
/* FP:lib.rs-1982 */             Note | OnceNote => "note",
/* FP:lib.rs-1983 */             Help | OnceHelp => "help",
/* FP:lib.rs-1984 */             FailureNote => "failure-note",
/* FP:lib.rs-1985 */             Allow | Expect => unreachable!(),
/* FP:lib.rs-1986 */         }
/* FP:lib.rs-1987 */     }
/* FP:lib.rs-1988 */ 
/* FP:lib.rs-1989 */     pub fn is_failure_note(&self) -> bool {
/* FP:lib.rs-1990 */         matches!(*self, FailureNote)
/* FP:lib.rs-1991 */     }
/* FP:lib.rs-1992 */ 
/* FP:lib.rs-1993 */     // Can this level be used in a subdiagnostic message?
/* FP:lib.rs-1994 */     fn can_be_subdiag(&self) -> bool {
/* FP:lib.rs-1995 */         match self {
/* FP:lib.rs-1996 */             Bug | DelayedBug | Fatal | Error | ForceWarning | FailureNote | Allow | Expect => false,
/* FP:lib.rs-1997 */ 
/* FP:lib.rs-1998 */             Warning | Note | Help | OnceNote | OnceHelp => true,
/* FP:lib.rs-1999 */         }
/* FP:lib.rs-2000 */     }
/* FP:lib.rs-2001 */ }
/* FP:lib.rs-2002 */ 
/* FP:lib.rs-2003 */ impl IntoDiagArg for Level {
/* FP:lib.rs-2004 */     fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
/* FP:lib.rs-2005 */         DiagArgValue::Str(Cow::from(self.to_string()))
/* FP:lib.rs-2006 */     }
/* FP:lib.rs-2007 */ }
/* FP:lib.rs-2008 */ 
/* FP:lib.rs-2009 */ // FIXME(eddyb) this doesn't belong here AFAICT, should be moved to callsite.
/* FP:lib.rs-2010 */ pub fn elided_lifetime_in_path_suggestion(
/* FP:lib.rs-2011 */     source_map: &SourceMap,
/* FP:lib.rs-2012 */     n: usize,
/* FP:lib.rs-2013 */     path_span: Span,
/* FP:lib.rs-2014 */     incl_angl_brckt: bool,
/* FP:lib.rs-2015 */     insertion_span: Span,
/* FP:lib.rs-2016 */ ) -> ElidedLifetimeInPathSubdiag {
/* FP:lib.rs-2017 */     let expected = ExpectedLifetimeParameter { span: path_span, count: n };
/* FP:lib.rs-2018 */     // Do not try to suggest anything if generated by a proc-macro.
/* FP:lib.rs-2019 */     let indicate = source_map.is_span_accessible(insertion_span).then(|| {
/* FP:lib.rs-2020 */         let anon_lts = vec!["'_"; n].join(", ");
/* FP:lib.rs-2021 */         let suggestion =
/* FP:lib.rs-2022 */             if incl_angl_brckt { format!("<{anon_lts}>") } else { format!("{anon_lts}, ") };
/* FP:lib.rs-2023 */ 
/* FP:lib.rs-2024 */         IndicateAnonymousLifetime { span: insertion_span.shrink_to_hi(), count: n, suggestion }
/* FP:lib.rs-2025 */     });
/* FP:lib.rs-2026 */ 
/* FP:lib.rs-2027 */     ElidedLifetimeInPathSubdiag { expected, indicate }
/* FP:lib.rs-2028 */ }
/* FP:lib.rs-2029 */ 
/* FP:lib.rs-2030 */ pub fn report_ambiguity_error<'a, G: EmissionGuarantee>(
/* FP:lib.rs-2031 */     diag: &mut Diag<'a, G>,
/* FP:lib.rs-2032 */     ambiguity: crate::rustc_lint_defs::AmbiguityErrorDiag,
/* FP:lib.rs-2033 */ ) {
/* FP:lib.rs-2034 */     diag.span_label(ambiguity.label_span, ambiguity.label_msg);
/* FP:lib.rs-2035 */     diag.note(ambiguity.note_msg);
/* FP:lib.rs-2036 */     diag.span_note(ambiguity.b1_span, ambiguity.b1_note_msg);
/* FP:lib.rs-2037 */     for help_msg in ambiguity.b1_help_msgs {
/* FP:lib.rs-2038 */         diag.help(help_msg);
/* FP:lib.rs-2039 */     }
/* FP:lib.rs-2040 */     diag.span_note(ambiguity.b2_span, ambiguity.b2_note_msg);
/* FP:lib.rs-2041 */     for help_msg in ambiguity.b2_help_msgs {
/* FP:lib.rs-2042 */         diag.help(help_msg);
/* FP:lib.rs-2043 */     }
/* FP:lib.rs-2044 */ }
/* FP:lib.rs-2045 */ 
/* FP:lib.rs-2046 */ /// Grammatical tool for displaying messages to end users in a nice form.
/* FP:lib.rs-2047 */ ///
/* FP:lib.rs-2048 */ /// Returns "an" if the given string starts with a vowel, and "a" otherwise.
/* FP:lib.rs-2049 */ pub fn a_or_an(s: &str) -> &'static str {
/* FP:lib.rs-2050 */     let mut chars = s.chars();
/* FP:lib.rs-2051 */     let Some(mut first_alpha_char) = chars.next() else {
/* FP:lib.rs-2052 */         return "a";
/* FP:lib.rs-2053 */     };
/* FP:lib.rs-2054 */     if first_alpha_char == '`' {
/* FP:lib.rs-2055 */         let Some(next) = chars.next() else {
/* FP:lib.rs-2056 */             return "a";
/* FP:lib.rs-2057 */         };
/* FP:lib.rs-2058 */         first_alpha_char = next;
/* FP:lib.rs-2059 */     }
/* FP:lib.rs-2060 */     if ["a", "e", "i", "o", "u", "&"].contains(&&first_alpha_char.to_lowercase().to_string()[..]) {
/* FP:lib.rs-2061 */         "an"
/* FP:lib.rs-2062 */     } else {
/* FP:lib.rs-2063 */         "a"
/* FP:lib.rs-2064 */     }
/* FP:lib.rs-2065 */ }
/* FP:lib.rs-2066 */ 
/* FP:lib.rs-2067 */ #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/* FP:lib.rs-2068 */ pub enum TerminalUrl {
/* FP:lib.rs-2069 */     No,
/* FP:lib.rs-2070 */     Yes,
/* FP:lib.rs-2071 */     Auto,
/* FP:lib.rs-2072 */ }