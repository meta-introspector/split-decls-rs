/* FP:lib.rs-0001 */ // Macro support for format strings
/* FP:lib.rs-0002 */ //
/* FP:lib.rs-0003 */ // These structures are used when parsing format strings for the compiler.
/* FP:lib.rs-0004 */ // Parsing does not happen at runtime: structures of `std::fmt::rt` are
/* FP:lib.rs-0005 */ // generated instead.
/* FP:lib.rs-0006 */ 
/* FP:lib.rs-0007 */ // tidy-alphabetical-start
/* FP:lib.rs-0008 */ // We want to be able to build this crate with a stable compiler,
/* FP:lib.rs-0009 */ // so no `#[feature]` attributes should be added.
/* FP:lib.rs-0010 */ #[deny(unstable_features)]
/* FP:lib.rs-0011 */ #[doc(
/* FP:lib.rs-0012 */     html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/",
/* FP:lib.rs-0013 */     html_playground_url = "https://play.rust-lang.org/",
/* FP:lib.rs-0014 */     test(attr(deny(warnings)))
/* FP:lib.rs-0015 */ )]
/* FP:lib.rs-0016 */ // tidy-alphabetical-end
/* FP:lib.rs-0017 */ 
/* FP:lib.rs-0018 */ use std::ops::Range;
/* FP:lib.rs-0019 */ 
/* FP:lib.rs-0020 */ pub use Alignment::*;
/* FP:lib.rs-0021 */ pub use Count::*;
/* FP:lib.rs-0022 */ pub use Position::*;
/* FP:lib.rs-0023 */ 
/* FP:lib.rs-0024 */ /// The type of format string that we are parsing.
/* FP:lib.rs-0025 */ #[derive(Copy, Clone, Debug, Eq, PartialEq)]
/* FP:lib.rs-0026 */ pub enum ParseMode {
/* FP:lib.rs-0027 */     /// A normal format string as per `format_args!`.
/* FP:lib.rs-0028 */     Format,
/* FP:lib.rs-0029 */     /// An inline assembly template string for `asm!`.
/* FP:lib.rs-0030 */     InlineAsm,
/* FP:lib.rs-0031 */     /// A format string for use in diagnostic attributes.
/* FP:lib.rs-0032 */     ///
/* FP:lib.rs-0033 */     /// Similar to `format_args!`, however only named ("captured") arguments
/* FP:lib.rs-0034 */     /// are allowed, and no format modifiers are permitted.
/* FP:lib.rs-0035 */     Diagnostic,
/* FP:lib.rs-0036 */ }
/* FP:lib.rs-0037 */ 
/* FP:lib.rs-0038 */ /// A piece is a portion of the format string which represents the next part
/* FP:lib.rs-0039 */ /// to emit. These are emitted as a stream by the `Parser` class.
/* FP:lib.rs-0040 */ #[derive(Clone, Debug, PartialEq)]
/* FP:lib.rs-0041 */ pub enum Piece<'input> {
/* FP:lib.rs-0042 */     /// A literal string which should directly be emitted
/* FP:lib.rs-0043 */     Lit(&'input str),
/* FP:lib.rs-0044 */     /// This describes that formatting should process the next argument (as
/* FP:lib.rs-0045 */     /// specified inside) for emission.
/* FP:lib.rs-0046 */     NextArgument(Box<Argument<'input>>),
/* FP:lib.rs-0047 */ }
/* FP:lib.rs-0048 */ 
/* FP:lib.rs-0049 */ /// Representation of an argument specification.
/* FP:lib.rs-0050 */ #[derive(Clone, Debug, PartialEq)]
/* FP:lib.rs-0051 */ pub struct Argument<'input> {
/* FP:lib.rs-0052 */     /// Where to find this argument
/* FP:lib.rs-0053 */     pub position: Position<'input>,
/* FP:lib.rs-0054 */     /// The span of the position indicator. Includes any whitespace in implicit
/* FP:lib.rs-0055 */     /// positions (`{  }`).
/* FP:lib.rs-0056 */     pub position_span: Range<usize>,
/* FP:lib.rs-0057 */     /// How to format the argument
/* FP:lib.rs-0058 */     pub format: FormatSpec<'input>,
/* FP:lib.rs-0059 */ }
/* FP:lib.rs-0060 */ 
/* FP:lib.rs-0061 */ impl<'input> Argument<'input> {
/* FP:lib.rs-0062 */     pub fn is_identifier(&self) -> bool {
/* FP:lib.rs-0063 */         matches!(self.position, Position::ArgumentNamed(_)) && self.format == FormatSpec::default()
/* FP:lib.rs-0064 */     }
/* FP:lib.rs-0065 */ }
/* FP:lib.rs-0066 */ 
/* FP:lib.rs-0067 */ /// Specification for the formatting of an argument in the format string.
/* FP:lib.rs-0068 */ #[derive(Clone, Debug, PartialEq, Default)]
/* FP:lib.rs-0069 */ pub struct FormatSpec<'input> {
/* FP:lib.rs-0070 */     /// Optionally specified character to fill alignment with.
/* FP:lib.rs-0071 */     pub fill: Option<char>,
/* FP:lib.rs-0072 */     /// Span of the optionally specified fill character.
/* FP:lib.rs-0073 */     pub fill_span: Option<Range<usize>>,
/* FP:lib.rs-0074 */     /// Optionally specified alignment.
/* FP:lib.rs-0075 */     pub align: Alignment,
/* FP:lib.rs-0076 */     /// The `+` or `-` flag.
/* FP:lib.rs-0077 */     pub sign: Option<Sign>,
/* FP:lib.rs-0078 */     /// The `#` flag.
/* FP:lib.rs-0079 */     pub alternate: bool,
/* FP:lib.rs-0080 */     /// The `0` flag.
/* FP:lib.rs-0081 */     pub zero_pad: bool,
/* FP:lib.rs-0082 */     /// The `x` or `X` flag. (Only for `Debug`.)
/* FP:lib.rs-0083 */     pub debug_hex: Option<DebugHex>,
/* FP:lib.rs-0084 */     /// The integer precision to use.
/* FP:lib.rs-0085 */     pub precision: Count<'input>,
/* FP:lib.rs-0086 */     /// The span of the precision formatting flag (for diagnostics).
/* FP:lib.rs-0087 */     pub precision_span: Option<Range<usize>>,
/* FP:lib.rs-0088 */     /// The string width requested for the resulting format.
/* FP:lib.rs-0089 */     pub width: Count<'input>,
/* FP:lib.rs-0090 */     /// The span of the width formatting flag (for diagnostics).
/* FP:lib.rs-0091 */     pub width_span: Option<Range<usize>>,
/* FP:lib.rs-0092 */     /// The descriptor string representing the name of the format desired for
/* FP:lib.rs-0093 */     /// this argument, this can be empty or any number of characters, although
/* FP:lib.rs-0094 */     /// it is required to be one word.
/* FP:lib.rs-0095 */     pub ty: &'input str,
/* FP:lib.rs-0096 */     /// The span of the descriptor string (for diagnostics).
/* FP:lib.rs-0097 */     pub ty_span: Option<Range<usize>>,
/* FP:lib.rs-0098 */ }
/* FP:lib.rs-0099 */ 
/* FP:lib.rs-0100 */ /// Enum describing where an argument for a format can be located.
/* FP:lib.rs-0101 */ #[derive(Clone, Debug, PartialEq)]
/* FP:lib.rs-0102 */ pub enum Position<'input> {
/* FP:lib.rs-0103 */     /// The argument is implied to be located at an index
/* FP:lib.rs-0104 */     ArgumentImplicitlyIs(usize),
/* FP:lib.rs-0105 */     /// The argument is located at a specific index given in the format,
/* FP:lib.rs-0106 */     ArgumentIs(usize),
/* FP:lib.rs-0107 */     /// The argument has a name.
/* FP:lib.rs-0108 */     ArgumentNamed(&'input str),
/* FP:lib.rs-0109 */ }
/* FP:lib.rs-0110 */ 
/* FP:lib.rs-0111 */ impl Position<'_> {
/* FP:lib.rs-0112 */     pub fn index(&self) -> Option<usize> {
/* FP:lib.rs-0113 */         match self {
/* FP:lib.rs-0114 */             ArgumentIs(i, ..) | ArgumentImplicitlyIs(i) => Some(*i),
/* FP:lib.rs-0115 */             _ => None,
/* FP:lib.rs-0116 */         }
/* FP:lib.rs-0117 */     }
/* FP:lib.rs-0118 */ }
/* FP:lib.rs-0119 */ 
/* FP:lib.rs-0120 */ /// Enum of alignments which are supported.
/* FP:lib.rs-0121 */ #[derive(Copy, Clone, Debug, PartialEq, Default)]
/* FP:lib.rs-0122 */ pub enum Alignment {
/* FP:lib.rs-0123 */     /// The value will be aligned to the left.
/* FP:lib.rs-0124 */     AlignLeft,
/* FP:lib.rs-0125 */     /// The value will be aligned to the right.
/* FP:lib.rs-0126 */     AlignRight,
/* FP:lib.rs-0127 */     /// The value will be aligned in the center.
/* FP:lib.rs-0128 */     AlignCenter,
/* FP:lib.rs-0129 */     /// The value will take on a default alignment.
/* FP:lib.rs-0130 */     #[default]
/* FP:lib.rs-0131 */     AlignUnknown,
/* FP:lib.rs-0132 */ }
/* FP:lib.rs-0133 */ 
/* FP:lib.rs-0134 */ /// Enum for the sign flags.
/* FP:lib.rs-0135 */ #[derive(Copy, Clone, Debug, PartialEq)]
/* FP:lib.rs-0136 */ pub enum Sign {
/* FP:lib.rs-0137 */     /// The `+` flag.
/* FP:lib.rs-0138 */     Plus,
/* FP:lib.rs-0139 */     /// The `-` flag.
/* FP:lib.rs-0140 */     Minus,
/* FP:lib.rs-0141 */ }
/* FP:lib.rs-0142 */ 
/* FP:lib.rs-0143 */ /// Enum for the debug hex flags.
/* FP:lib.rs-0144 */ #[derive(Copy, Clone, Debug, PartialEq)]
/* FP:lib.rs-0145 */ pub enum DebugHex {
/* FP:lib.rs-0146 */     /// The `x` flag in `{:x?}`.
/* FP:lib.rs-0147 */     Lower,
/* FP:lib.rs-0148 */     /// The `X` flag in `{:X?}`.
/* FP:lib.rs-0149 */     Upper,
/* FP:lib.rs-0150 */ }
/* FP:lib.rs-0151 */ 
/* FP:lib.rs-0152 */ /// A count is used for the precision and width parameters of an integer, and
/* FP:lib.rs-0153 */ /// can reference either an argument or a literal integer.
/* FP:lib.rs-0154 */ #[derive(Clone, Debug, PartialEq, Default)]
/* FP:lib.rs-0155 */ pub enum Count<'input> {
/* FP:lib.rs-0156 */     /// The count is specified explicitly.
/* FP:lib.rs-0157 */     CountIs(u16),
/* FP:lib.rs-0158 */     /// The count is specified by the argument with the given name.
/* FP:lib.rs-0159 */     CountIsName(&'input str, Range<usize>),
/* FP:lib.rs-0160 */     /// The count is specified by the argument at the given index.
/* FP:lib.rs-0161 */     CountIsParam(usize),
/* FP:lib.rs-0162 */     /// The count is specified by a star (like in `{:.*}`) that refers to the argument at the given index.
/* FP:lib.rs-0163 */     CountIsStar(usize),
/* FP:lib.rs-0164 */     /// The count is implied and cannot be explicitly specified.
/* FP:lib.rs-0165 */     #[default]
/* FP:lib.rs-0166 */     CountImplied,
/* FP:lib.rs-0167 */ }
/* FP:lib.rs-0168 */ 
/* FP:lib.rs-0169 */ pub struct ParseError {
/* FP:lib.rs-0170 */     pub description: String,
/* FP:lib.rs-0171 */     pub note: Option<String>,
/* FP:lib.rs-0172 */     pub label: String,
/* FP:lib.rs-0173 */     pub span: Range<usize>,
/* FP:lib.rs-0174 */     pub secondary_label: Option<(String, Range<usize>)>,
/* FP:lib.rs-0175 */     pub suggestion: Suggestion,
/* FP:lib.rs-0176 */ }
/* FP:lib.rs-0177 */ 
/* FP:lib.rs-0178 */ pub enum Suggestion {
/* FP:lib.rs-0179 */     None,
/* FP:lib.rs-0180 */     /// Replace inline argument with positional argument:
/* FP:lib.rs-0181 */     /// `format!("{foo.bar}")` -> `format!("{}", foo.bar)`
/* FP:lib.rs-0182 */     UsePositional,
/* FP:lib.rs-0183 */     /// Remove `r#` from identifier:
/* FP:lib.rs-0184 */     /// `format!("{r#foo}")` -> `format!("{foo}")`
/* FP:lib.rs-0185 */     RemoveRawIdent(Range<usize>),
/* FP:lib.rs-0186 */     /// Reorder format parameter:
/* FP:lib.rs-0187 */     /// `format!("{foo:?#}")` -> `format!("{foo:#?}")`
/* FP:lib.rs-0188 */     /// `format!("{foo:?x}")` -> `format!("{foo:x?}")`
/* FP:lib.rs-0189 */     /// `format!("{foo:?X}")` -> `format!("{foo:X?}")`
/* FP:lib.rs-0190 */     ReorderFormatParameter(Range<usize>, String),
/* FP:lib.rs-0191 */ }
/* FP:lib.rs-0192 */ 
/* FP:lib.rs-0193 */ /// The parser structure for interpreting the input format string. This is
/* FP:lib.rs-0194 */ /// modeled as an iterator over `Piece` structures to form a stream of tokens
/* FP:lib.rs-0195 */ /// being output.
/* FP:lib.rs-0196 */ ///
/* FP:lib.rs-0197 */ /// This is a recursive-descent parser for the sake of simplicity, and if
/* FP:lib.rs-0198 */ /// necessary there's probably lots of room for improvement performance-wise.
/* FP:lib.rs-0199 */ pub struct Parser<'input> {
/* FP:lib.rs-0200 */     mode: ParseMode,
/* FP:lib.rs-0201 */     /// Input to be parsed
/* FP:lib.rs-0202 */     input: &'input str,
/* FP:lib.rs-0203 */     /// Tuples of the span in the code snippet (input as written before being unescaped), the pos in input, and the char in input
/* FP:lib.rs-0204 */     input_vec: Vec<(Range<usize>, usize, char)>,
/* FP:lib.rs-0205 */     /// Index into input_vec
/* FP:lib.rs-0206 */     input_vec_index: usize,
/* FP:lib.rs-0207 */     /// Error messages accumulated during parsing
/* FP:lib.rs-0208 */     pub errors: Vec<ParseError>,
/* FP:lib.rs-0209 */     /// Current position of implicit positional argument pointer
/* FP:lib.rs-0210 */     pub curarg: usize,
/* FP:lib.rs-0211 */     /// Start and end byte offset of every successfully parsed argument
/* FP:lib.rs-0212 */     pub arg_places: Vec<Range<usize>>,
/* FP:lib.rs-0213 */     /// Span of the last opening brace seen, used for error reporting
/* FP:lib.rs-0214 */     last_open_brace: Option<Range<usize>>,
/* FP:lib.rs-0215 */     /// Whether this formatting string was written directly in the source. This controls whether we
/* FP:lib.rs-0216 */     /// can use spans to refer into it and give better error messages.
/* FP:lib.rs-0217 */     /// N.B: This does _not_ control whether implicit argument captures can be used.
/* FP:lib.rs-0218 */     pub is_source_literal: bool,
/* FP:lib.rs-0219 */     /// Index to the end of the literal snippet
/* FP:lib.rs-0220 */     end_of_snippet: usize,
/* FP:lib.rs-0221 */     /// Start position of the current line.
/* FP:lib.rs-0222 */     cur_line_start: usize,
/* FP:lib.rs-0223 */     /// Start and end byte offset of every line of the format string. Excludes
/* FP:lib.rs-0224 */     /// newline characters and leading whitespace.
/* FP:lib.rs-0225 */     pub line_spans: Vec<Range<usize>>,
/* FP:lib.rs-0226 */ }
/* FP:lib.rs-0227 */ 
/* FP:lib.rs-0228 */ impl<'input> Iterator for Parser<'input> {
/* FP:lib.rs-0229 */     type Item = Piece<'input>;
/* FP:lib.rs-0230 */ 
/* FP:lib.rs-0231 */     fn next(&mut self) -> Option<Piece<'input>> {
/* FP:lib.rs-0232 */         if let Some((Range { start, end }, idx, ch)) = self.peek() {
/* FP:lib.rs-0233 */             match ch {
/* FP:lib.rs-0234 */                 '{' => {
/* FP:lib.rs-0235 */                     self.input_vec_index += 1;
/* FP:lib.rs-0236 */                     if let Some((_, i, '{')) = self.peek() {
/* FP:lib.rs-0237 */                         self.input_vec_index += 1;
/* FP:lib.rs-0238 */                         // double open brace escape: "{{"
/* FP:lib.rs-0239 */                         // next state after this is either end-of-input or seen-a-brace
/* FP:lib.rs-0240 */                         Some(Piece::Lit(self.string(i)))
/* FP:lib.rs-0241 */                     } else {
/* FP:lib.rs-0242 */                         // single open brace
/* FP:lib.rs-0243 */                         self.last_open_brace = Some(start..end);
/* FP:lib.rs-0244 */                         let arg = self.argument();
/* FP:lib.rs-0245 */                         self.ws();
/* FP:lib.rs-0246 */                         if let Some((close_brace_range, _)) = self.consume_pos('}') {
/* FP:lib.rs-0247 */                             if self.is_source_literal {
/* FP:lib.rs-0248 */                                 self.arg_places.push(start..close_brace_range.end);
/* FP:lib.rs-0249 */                             }
/* FP:lib.rs-0250 */                         } else {
/* FP:lib.rs-0251 */                             self.missing_closing_brace(&arg);
/* FP:lib.rs-0252 */                         }
/* FP:lib.rs-0253 */ 
/* FP:lib.rs-0254 */                         Some(Piece::NextArgument(Box::new(arg)))
/* FP:lib.rs-0255 */                     }
/* FP:lib.rs-0256 */                 }
/* FP:lib.rs-0257 */                 '}' => {
/* FP:lib.rs-0258 */                     self.input_vec_index += 1;
/* FP:lib.rs-0259 */                     if let Some((_, i, '}')) = self.peek() {
/* FP:lib.rs-0260 */                         self.input_vec_index += 1;
/* FP:lib.rs-0261 */                         // double close brace escape: "}}"
/* FP:lib.rs-0262 */                         // next state after this is either end-of-input or start
/* FP:lib.rs-0263 */                         Some(Piece::Lit(self.string(i)))
/* FP:lib.rs-0264 */                     } else {
/* FP:lib.rs-0265 */                         // error: single close brace without corresponding open brace
/* FP:lib.rs-0266 */                         self.errors.push(ParseError {
/* FP:lib.rs-0267 */                             description: "unmatched `}` found".into(),
/* FP:lib.rs-0268 */                             note: Some(
/* FP:lib.rs-0269 */                                 "if you intended to print `}`, you can escape it using `}}`".into(),
/* FP:lib.rs-0270 */                             ),
/* FP:lib.rs-0271 */                             label: "unmatched `}`".into(),
/* FP:lib.rs-0272 */                             span: start..end,
/* FP:lib.rs-0273 */                             secondary_label: None,
/* FP:lib.rs-0274 */                             suggestion: Suggestion::None,
/* FP:lib.rs-0275 */                         });
/* FP:lib.rs-0276 */                         None
/* FP:lib.rs-0277 */                     }
/* FP:lib.rs-0278 */                 }
/* FP:lib.rs-0279 */                 _ => Some(Piece::Lit(self.string(idx))),
/* FP:lib.rs-0280 */             }
/* FP:lib.rs-0281 */         } else {
/* FP:lib.rs-0282 */             // end of input
/* FP:lib.rs-0283 */             if self.is_source_literal {
/* FP:lib.rs-0284 */                 let span = self.cur_line_start..self.end_of_snippet;
/* FP:lib.rs-0285 */                 if self.line_spans.last() != Some(&span) {
/* FP:lib.rs-0286 */                     self.line_spans.push(span);
/* FP:lib.rs-0287 */                 }
/* FP:lib.rs-0288 */             }
/* FP:lib.rs-0289 */             None
/* FP:lib.rs-0290 */         }
/* FP:lib.rs-0291 */     }
/* FP:lib.rs-0292 */ }
/* FP:lib.rs-0293 */ 
/* FP:lib.rs-0294 */ impl<'input> Parser<'input> {
/* FP:lib.rs-0295 */     /// Creates a new parser for the given unescaped input string and
/* FP:lib.rs-0296 */     /// optional code snippet (the input as written before being unescaped),
/* FP:lib.rs-0297 */     /// where `style` is `Some(nr_hashes)` when the snippet is a raw string with that many hashes.
/* FP:lib.rs-0298 */     /// If the input comes via `println` or `panic`, then it has a newline already appended,
/* FP:lib.rs-0299 */     /// which is reflected in the `appended_newline` parameter.
/* FP:lib.rs-0300 */     pub fn new(
/* FP:lib.rs-0301 */         input: &'input str,
/* FP:lib.rs-0302 */         style: Option<usize>,
/* FP:lib.rs-0303 */         snippet: Option<String>,
/* FP:lib.rs-0304 */         appended_newline: bool,
/* FP:lib.rs-0305 */         mode: ParseMode,
/* FP:lib.rs-0306 */     ) -> Self {
/* FP:lib.rs-0307 */         let quote_offset = style.map_or(1, |nr_hashes| nr_hashes + 2);
/* FP:lib.rs-0308 */ 
/* FP:lib.rs-0309 */         let (is_source_literal, end_of_snippet, pre_input_vec) = if let Some(snippet) = snippet {
/* FP:lib.rs-0310 */             if let Some(nr_hashes) = style {
/* FP:lib.rs-0311 */                 // snippet is a raw string, which starts with 'r', a number of hashes, and a quote
/* FP:lib.rs-0312 */                 // and ends with a quote and the same number of hashes
/* FP:lib.rs-0313 */                 (true, snippet.len() - nr_hashes - 1, vec![])
/* FP:lib.rs-0314 */             } else {
/* FP:lib.rs-0315 */                 // snippet is not a raw string
/* FP:lib.rs-0316 */                 if snippet.starts_with('"') {
/* FP:lib.rs-0317 */                     // snippet looks like an ordinary string literal
/* FP:lib.rs-0318 */                     // check whether it is the escaped version of input
/* FP:lib.rs-0319 */                     let without_quotes = &snippet[1..snippet.len() - 1];
/* FP:lib.rs-0320 */                     let (mut ok, mut vec) = (true, vec![]);
/* FP:lib.rs-0321 */                     let mut chars = input.chars();
/* FP:lib.rs-0322 */                     rustc_literal_escaper::unescape_str(without_quotes, |range, res| match res {
/* FP:lib.rs-0323 */                         Ok(ch) if ok && chars.next().is_some_and(|c| ch == c) => {
/* FP:lib.rs-0324 */                             vec.push((range, ch));
/* FP:lib.rs-0325 */                         }
/* FP:lib.rs-0326 */                         _ => {
/* FP:lib.rs-0327 */                             ok = false;
/* FP:lib.rs-0328 */                             vec = vec![];
/* FP:lib.rs-0329 */                         }
/* FP:lib.rs-0330 */                     });
/* FP:lib.rs-0331 */                     let end = vec.last().map(|(r, _)| r.end).unwrap_or(0);
/* FP:lib.rs-0332 */                     if ok {
/* FP:lib.rs-0333 */                         if appended_newline {
/* FP:lib.rs-0334 */                             if chars.as_str() == "\n" {
/* FP:lib.rs-0335 */                                 vec.push((end..end + 1, '\n'));
/* FP:lib.rs-0336 */                                 (true, 1 + end, vec)
/* FP:lib.rs-0337 */                             } else {
/* FP:lib.rs-0338 */                                 (false, snippet.len(), vec![])
/* FP:lib.rs-0339 */                             }
/* FP:lib.rs-0340 */                         } else if chars.as_str() == "" {
/* FP:lib.rs-0341 */                             (true, 1 + end, vec)
/* FP:lib.rs-0342 */                         } else {
/* FP:lib.rs-0343 */                             (false, snippet.len(), vec![])
/* FP:lib.rs-0344 */                         }
/* FP:lib.rs-0345 */                     } else {
/* FP:lib.rs-0346 */                         (false, snippet.len(), vec![])
/* FP:lib.rs-0347 */                     }
/* FP:lib.rs-0348 */                 } else {
/* FP:lib.rs-0349 */                     // snippet is not a raw string and does not start with '"'
/* FP:lib.rs-0350 */                     (false, snippet.len(), vec![])
/* FP:lib.rs-0351 */                 }
/* FP:lib.rs-0352 */             }
/* FP:lib.rs-0353 */         } else {
/* FP:lib.rs-0354 */             // snippet is None
/* FP:lib.rs-0355 */             (false, input.len() - if appended_newline { 1 } else { 0 }, vec![])
/* FP:lib.rs-0356 */         };
/* FP:lib.rs-0357 */ 
/* FP:lib.rs-0358 */         let input_vec: Vec<(Range<usize>, usize, char)> = if pre_input_vec.is_empty() {
/* FP:lib.rs-0359 */             // Snippet is *not* input before unescaping, so spans pointing at it will be incorrect.
/* FP:lib.rs-0360 */             // This can happen with proc macros that respan generated literals.
/* FP:lib.rs-0361 */             input
/* FP:lib.rs-0362 */                 .char_indices()
/* FP:lib.rs-0363 */                 .map(|(idx, c)| {
/* FP:lib.rs-0364 */                     let i = idx + quote_offset;
/* FP:lib.rs-0365 */                     (i..i + c.len_utf8(), idx, c)
/* FP:lib.rs-0366 */                 })
/* FP:lib.rs-0367 */                 .collect()
/* FP:lib.rs-0368 */         } else {
/* FP:lib.rs-0369 */             // Snippet is input before unescaping
/* FP:lib.rs-0370 */             input
/* FP:lib.rs-0371 */                 .char_indices()
/* FP:lib.rs-0372 */                 .zip(pre_input_vec)
/* FP:lib.rs-0373 */                 .map(|((i, c), (r, _))| (r.start + quote_offset..r.end + quote_offset, i, c))
/* FP:lib.rs-0374 */                 .collect()
/* FP:lib.rs-0375 */         };
/* FP:lib.rs-0376 */ 
/* FP:lib.rs-0377 */         Parser {
/* FP:lib.rs-0378 */             mode,
/* FP:lib.rs-0379 */             input,
/* FP:lib.rs-0380 */             input_vec,
/* FP:lib.rs-0381 */             input_vec_index: 0,
/* FP:lib.rs-0382 */             errors: vec![],
/* FP:lib.rs-0383 */             curarg: 0,
/* FP:lib.rs-0384 */             arg_places: vec![],
/* FP:lib.rs-0385 */             last_open_brace: None,
/* FP:lib.rs-0386 */             is_source_literal,
/* FP:lib.rs-0387 */             end_of_snippet,
/* FP:lib.rs-0388 */             cur_line_start: quote_offset,
/* FP:lib.rs-0389 */             line_spans: vec![],
/* FP:lib.rs-0390 */         }
/* FP:lib.rs-0391 */     }
/* FP:lib.rs-0392 */ 
/* FP:lib.rs-0393 */     /// Peeks at the current position, without incrementing the pointer.
/* FP:lib.rs-0394 */     pub fn peek(&self) -> Option<(Range<usize>, usize, char)> {
/* FP:lib.rs-0395 */         self.input_vec.get(self.input_vec_index).cloned()
/* FP:lib.rs-0396 */     }
/* FP:lib.rs-0397 */ 
/* FP:lib.rs-0398 */     /// Peeks at the current position + 1, without incrementing the pointer.
/* FP:lib.rs-0399 */     pub fn peek_ahead(&self) -> Option<(Range<usize>, usize, char)> {
/* FP:lib.rs-0400 */         self.input_vec.get(self.input_vec_index + 1).cloned()
/* FP:lib.rs-0401 */     }
/* FP:lib.rs-0402 */ 
/* FP:lib.rs-0403 */     /// Optionally consumes the specified character. If the character is not at
/* FP:lib.rs-0404 */     /// the current position, then the current iterator isn't moved and `false` is
/* FP:lib.rs-0405 */     /// returned, otherwise the character is consumed and `true` is returned.
/* FP:lib.rs-0406 */     fn consume(&mut self, c: char) -> bool {
/* FP:lib.rs-0407 */         self.consume_pos(c).is_some()
/* FP:lib.rs-0408 */     }
/* FP:lib.rs-0409 */ 
/* FP:lib.rs-0410 */     /// Optionally consumes the specified character. If the character is not at
/* FP:lib.rs-0411 */     /// the current position, then the current iterator isn't moved and `None` is
/* FP:lib.rs-0412 */     /// returned, otherwise the character is consumed and the current position is
/* FP:lib.rs-0413 */     /// returned.
/* FP:lib.rs-0414 */     fn consume_pos(&mut self, ch: char) -> Option<(Range<usize>, usize)> {
/* FP:lib.rs-0415 */         if let Some((r, i, c)) = self.peek()
/* FP:lib.rs-0416 */             && ch == c
/* FP:lib.rs-0417 */         {
/* FP:lib.rs-0418 */             self.input_vec_index += 1;
/* FP:lib.rs-0419 */             return Some((r, i));
/* FP:lib.rs-0420 */         }
/* FP:lib.rs-0421 */ 
/* FP:lib.rs-0422 */         None
/* FP:lib.rs-0423 */     }
/* FP:lib.rs-0424 */ 
/* FP:lib.rs-0425 */     /// Called if a closing brace was not found.
/* FP:lib.rs-0426 */     fn missing_closing_brace(&mut self, arg: &Argument<'_>) {
/* FP:lib.rs-0427 */         let (range, description) = if let Some((r, _, c)) = self.peek() {
/* FP:lib.rs-0428 */             (r.start..r.start, format!("expected `}}`, found `{}`", c.escape_debug()))
/* FP:lib.rs-0429 */         } else {
/* FP:lib.rs-0430 */             (
/* FP:lib.rs-0431 */                 // point at closing `"`
/* FP:lib.rs-0432 */                 self.end_of_snippet..self.end_of_snippet,
/* FP:lib.rs-0433 */                 "expected `}` but string was terminated".to_owned(),
/* FP:lib.rs-0434 */             )
/* FP:lib.rs-0435 */         };
/* FP:lib.rs-0436 */ 
/* FP:lib.rs-0437 */         let (note, secondary_label) = if arg.format.fill == Some('}') {
/* FP:lib.rs-0438 */             (
/* FP:lib.rs-0439 */                 Some("the character `}` is interpreted as a fill character because of the `:` that precedes it".to_owned()),
/* FP:lib.rs-0440 */                 arg.format.fill_span.clone().map(|sp| ("this is not interpreted as a formatting closing brace".to_owned(), sp)),
/* FP:lib.rs-0441 */             )
/* FP:lib.rs-0442 */         } else {
/* FP:lib.rs-0443 */             (
/* FP:lib.rs-0444 */                 Some("if you intended to print `{`, you can escape it using `{{`".to_owned()),
/* FP:lib.rs-0445 */                 self.last_open_brace
/* FP:lib.rs-0446 */                     .clone()
/* FP:lib.rs-0447 */                     .map(|sp| ("because of this opening brace".to_owned(), sp)),
/* FP:lib.rs-0448 */             )
/* FP:lib.rs-0449 */         };
/* FP:lib.rs-0450 */ 
/* FP:lib.rs-0451 */         self.errors.push(ParseError {
/* FP:lib.rs-0452 */             description,
/* FP:lib.rs-0453 */             note,
/* FP:lib.rs-0454 */             label: "expected `}`".to_owned(),
/* FP:lib.rs-0455 */             span: range.start..range.start,
/* FP:lib.rs-0456 */             secondary_label,
/* FP:lib.rs-0457 */             suggestion: Suggestion::None,
/* FP:lib.rs-0458 */         });
/* FP:lib.rs-0459 */ 
/* FP:lib.rs-0460 */         if let Some((_, _, c)) = self.peek() {
/* FP:lib.rs-0461 */             match c {
/* FP:lib.rs-0462 */                 '?' => self.suggest_format_debug(),
/* FP:lib.rs-0463 */                 '<' | '^' | '>' => self.suggest_format_align(c),
/* FP:lib.rs-0464 */                 _ => self.suggest_positional_arg_instead_of_captured_arg(arg),
/* FP:lib.rs-0465 */             }
/* FP:lib.rs-0466 */         }
/* FP:lib.rs-0467 */     }
/* FP:lib.rs-0468 */ 
/* FP:lib.rs-0469 */     /// Consumes all whitespace characters until the first non-whitespace character
/* FP:lib.rs-0470 */     fn ws(&mut self) {
/* FP:lib.rs-0471 */         let rest = &self.input_vec[self.input_vec_index..];
/* FP:lib.rs-0472 */         let step = rest.iter().position(|&(_, _, c)| !c.is_whitespace()).unwrap_or(rest.len());
/* FP:lib.rs-0473 */         self.input_vec_index += step;
/* FP:lib.rs-0474 */     }
/* FP:lib.rs-0475 */ 
/* FP:lib.rs-0476 */     /// Parses all of a string which is to be considered a "raw literal" in a
/* FP:lib.rs-0477 */     /// format string. This is everything outside of the braces.
/* FP:lib.rs-0478 */     fn string(&mut self, start: usize) -> &'input str {
/* FP:lib.rs-0479 */         while let Some((r, i, c)) = self.peek() {
/* FP:lib.rs-0480 */             match c {
/* FP:lib.rs-0481 */                 '{' | '}' => {
/* FP:lib.rs-0482 */                     return &self.input[start..i];
/* FP:lib.rs-0483 */                 }
/* FP:lib.rs-0484 */                 '\n' if self.is_source_literal => {
/* FP:lib.rs-0485 */                     self.input_vec_index += 1;
/* FP:lib.rs-0486 */                     self.line_spans.push(self.cur_line_start..r.start);
/* FP:lib.rs-0487 */                     self.cur_line_start = r.end;
/* FP:lib.rs-0488 */                 }
/* FP:lib.rs-0489 */                 _ => {
/* FP:lib.rs-0490 */                     self.input_vec_index += 1;
/* FP:lib.rs-0491 */                     if self.is_source_literal && r.start == self.cur_line_start && c.is_whitespace()
/* FP:lib.rs-0492 */                     {
/* FP:lib.rs-0493 */                         self.cur_line_start = r.end;
/* FP:lib.rs-0494 */                     }
/* FP:lib.rs-0495 */                 }
/* FP:lib.rs-0496 */             }
/* FP:lib.rs-0497 */         }
/* FP:lib.rs-0498 */         &self.input[start..]
/* FP:lib.rs-0499 */     }
/* FP:lib.rs-0500 */ 
/* FP:lib.rs-0501 */     /// Parses an `Argument` structure, or what's contained within braces inside the format string.
/* FP:lib.rs-0502 */     fn argument(&mut self) -> Argument<'input> {
/* FP:lib.rs-0503 */         let start_idx = self.input_vec_index;
/* FP:lib.rs-0504 */ 
/* FP:lib.rs-0505 */         let position = self.position();
/* FP:lib.rs-0506 */         self.ws();
/* FP:lib.rs-0507 */ 
/* FP:lib.rs-0508 */         let end_idx = self.input_vec_index;
/* FP:lib.rs-0509 */ 
/* FP:lib.rs-0510 */         let format = match self.mode {
/* FP:lib.rs-0511 */             ParseMode::Format => self.format(),
/* FP:lib.rs-0512 */             ParseMode::InlineAsm => self.inline_asm(),
/* FP:lib.rs-0513 */             ParseMode::Diagnostic => self.diagnostic(),
/* FP:lib.rs-0514 */         };
/* FP:lib.rs-0515 */ 
/* FP:lib.rs-0516 */         // Resolve position after parsing format spec.
/* FP:lib.rs-0517 */         let position = position.unwrap_or_else(|| {
/* FP:lib.rs-0518 */             let i = self.curarg;
/* FP:lib.rs-0519 */             self.curarg += 1;
/* FP:lib.rs-0520 */             ArgumentImplicitlyIs(i)
/* FP:lib.rs-0521 */         });
/* FP:lib.rs-0522 */ 
/* FP:lib.rs-0523 */         let position_span =
/* FP:lib.rs-0524 */             self.input_vec_index2range(start_idx).start..self.input_vec_index2range(end_idx).start;
/* FP:lib.rs-0525 */         Argument { position, position_span, format }
/* FP:lib.rs-0526 */     }
/* FP:lib.rs-0527 */ 
/* FP:lib.rs-0528 */     /// Parses a positional argument for a format. This could either be an
/* FP:lib.rs-0529 */     /// integer index of an argument, a named argument, or a blank string.
/* FP:lib.rs-0530 */     /// Returns `Some(parsed_position)` if the position is not implicitly
/* FP:lib.rs-0531 */     /// consuming a macro argument, `None` if it's the case.
/* FP:lib.rs-0532 */     fn position(&mut self) -> Option<Position<'input>> {
/* FP:lib.rs-0533 */         if let Some(i) = self.integer() {
/* FP:lib.rs-0534 */             Some(ArgumentIs(i.into()))
/* FP:lib.rs-0535 */         } else {
/* FP:lib.rs-0536 */             match self.peek() {
/* FP:lib.rs-0537 */                 Some((range, _, c)) if rustc_lexer::is_id_start(c) => {
/* FP:lib.rs-0538 */                     let start = range.start;
/* FP:lib.rs-0539 */                     let word = self.word();
/* FP:lib.rs-0540 */ 
/* FP:lib.rs-0541 */                     // Recover from `r#ident` in format strings.
/* FP:lib.rs-0542 */                     if word == "r"
/* FP:lib.rs-0543 */                         && let Some((r, _, '#')) = self.peek()
/* FP:lib.rs-0544 */                         && self.peek_ahead().is_some_and(|(_, _, c)| rustc_lexer::is_id_start(c))
/* FP:lib.rs-0545 */                     {
/* FP:lib.rs-0546 */                         self.input_vec_index += 1;
/* FP:lib.rs-0547 */                         let prefix_end = r.end;
/* FP:lib.rs-0548 */                         let word = self.word();
/* FP:lib.rs-0549 */                         let prefix_span = start..prefix_end;
/* FP:lib.rs-0550 */                         let full_span =
/* FP:lib.rs-0551 */                             start..self.input_vec_index2range(self.input_vec_index).start;
/* FP:lib.rs-0552 */                         self.errors.insert(0, ParseError {
/* FP:lib.rs-0553 */                                     description: "raw identifiers are not supported".to_owned(),
/* FP:lib.rs-0554 */                                     note: Some("identifiers in format strings can be keywords and don't need to be prefixed with `r#`".to_string()),
/* FP:lib.rs-0555 */                                     label: "raw identifier used here".to_owned(),
/* FP:lib.rs-0556 */                                     span: full_span,
/* FP:lib.rs-0557 */                                     secondary_label: None,
/* FP:lib.rs-0558 */                                     suggestion: Suggestion::RemoveRawIdent(prefix_span),
/* FP:lib.rs-0559 */                                 });
/* FP:lib.rs-0560 */                         return Some(ArgumentNamed(word));
/* FP:lib.rs-0561 */                     }
/* FP:lib.rs-0562 */ 
/* FP:lib.rs-0563 */                     Some(ArgumentNamed(word))
/* FP:lib.rs-0564 */                 }
/* FP:lib.rs-0565 */                 // This is an `ArgumentNext`.
/* FP:lib.rs-0566 */                 // Record the fact and do the resolution after parsing the
/* FP:lib.rs-0567 */                 // format spec, to make things like `{:.*}` work.
/* FP:lib.rs-0568 */                 _ => None,
/* FP:lib.rs-0569 */             }
/* FP:lib.rs-0570 */         }
/* FP:lib.rs-0571 */     }
/* FP:lib.rs-0572 */ 
/* FP:lib.rs-0573 */     fn input_vec_index2pos(&self, index: usize) -> usize {
/* FP:lib.rs-0574 */         if let Some((_, pos, _)) = self.input_vec.get(index) { *pos } else { self.input.len() }
/* FP:lib.rs-0575 */     }
/* FP:lib.rs-0576 */ 
/* FP:lib.rs-0577 */     fn input_vec_index2range(&self, index: usize) -> Range<usize> {
/* FP:lib.rs-0578 */         if let Some((r, _, _)) = self.input_vec.get(index) {
/* FP:lib.rs-0579 */             r.clone()
/* FP:lib.rs-0580 */         } else {
/* FP:lib.rs-0581 */             self.end_of_snippet..self.end_of_snippet
/* FP:lib.rs-0582 */         }
/* FP:lib.rs-0583 */     }
/* FP:lib.rs-0584 */ 
/* FP:lib.rs-0585 */     /// Parses a format specifier at the current position, returning all of the
/* FP:lib.rs-0586 */     /// relevant information in the `FormatSpec` struct.
/* FP:lib.rs-0587 */     fn format(&mut self) -> FormatSpec<'input> {
/* FP:lib.rs-0588 */         let mut spec = FormatSpec::default();
/* FP:lib.rs-0589 */ 
/* FP:lib.rs-0590 */         if !self.consume(':') {
/* FP:lib.rs-0591 */             return spec;
/* FP:lib.rs-0592 */         }
/* FP:lib.rs-0593 */ 
/* FP:lib.rs-0594 */         // fill character
/* FP:lib.rs-0595 */         if let (Some((r, _, c)), Some((_, _, '>' | '<' | '^'))) = (self.peek(), self.peek_ahead()) {
/* FP:lib.rs-0596 */             self.input_vec_index += 1;
/* FP:lib.rs-0597 */             spec.fill = Some(c);
/* FP:lib.rs-0598 */             spec.fill_span = Some(r);
/* FP:lib.rs-0599 */         }
/* FP:lib.rs-0600 */         // Alignment
/* FP:lib.rs-0601 */         if self.consume('<') {
/* FP:lib.rs-0602 */             spec.align = AlignLeft;
/* FP:lib.rs-0603 */         } else if self.consume('>') {
/* FP:lib.rs-0604 */             spec.align = AlignRight;
/* FP:lib.rs-0605 */         } else if self.consume('^') {
/* FP:lib.rs-0606 */             spec.align = AlignCenter;
/* FP:lib.rs-0607 */         }
/* FP:lib.rs-0608 */         // Sign flags
/* FP:lib.rs-0609 */         if self.consume('+') {
/* FP:lib.rs-0610 */             spec.sign = Some(Sign::Plus);
/* FP:lib.rs-0611 */         } else if self.consume('-') {
/* FP:lib.rs-0612 */             spec.sign = Some(Sign::Minus);
/* FP:lib.rs-0613 */         }
/* FP:lib.rs-0614 */         // Alternate marker
/* FP:lib.rs-0615 */         if self.consume('#') {
/* FP:lib.rs-0616 */             spec.alternate = true;
/* FP:lib.rs-0617 */         }
/* FP:lib.rs-0618 */         // Width and precision
/* FP:lib.rs-0619 */         let mut havewidth = false;
/* FP:lib.rs-0620 */ 
/* FP:lib.rs-0621 */         if let Some((range, _)) = self.consume_pos('0') {
/* FP:lib.rs-0622 */             // small ambiguity with '0$' as a format string. In theory this is a
/* FP:lib.rs-0623 */             // '0' flag and then an ill-formatted format string with just a '$'
/* FP:lib.rs-0624 */             // and no count, but this is better if we instead interpret this as
/* FP:lib.rs-0625 */             // no '0' flag and '0$' as the width instead.
/* FP:lib.rs-0626 */             if let Some((r, _)) = self.consume_pos('$') {
/* FP:lib.rs-0627 */                 spec.width = CountIsParam(0);
/* FP:lib.rs-0628 */                 spec.width_span = Some(range.start..r.end);
/* FP:lib.rs-0629 */                 havewidth = true;
/* FP:lib.rs-0630 */             } else {
/* FP:lib.rs-0631 */                 spec.zero_pad = true;
/* FP:lib.rs-0632 */             }
/* FP:lib.rs-0633 */         }
/* FP:lib.rs-0634 */ 
/* FP:lib.rs-0635 */         if !havewidth {
/* FP:lib.rs-0636 */             let start_idx = self.input_vec_index;
/* FP:lib.rs-0637 */             spec.width = self.count();
/* FP:lib.rs-0638 */             if spec.width != CountImplied {
/* FP:lib.rs-0639 */                 let end = self.input_vec_index2range(self.input_vec_index).start;
/* FP:lib.rs-0640 */                 spec.width_span = Some(self.input_vec_index2range(start_idx).start..end);
/* FP:lib.rs-0641 */             }
/* FP:lib.rs-0642 */         }
/* FP:lib.rs-0643 */ 
/* FP:lib.rs-0644 */         if let Some((range, _)) = self.consume_pos('.') {
/* FP:lib.rs-0645 */             if self.consume('*') {
/* FP:lib.rs-0646 */                 // Resolve `CountIsNextParam`.
/* FP:lib.rs-0647 */                 // We can do this immediately as `position` is resolved later.
/* FP:lib.rs-0648 */                 let i = self.curarg;
/* FP:lib.rs-0649 */                 self.curarg += 1;
/* FP:lib.rs-0650 */                 spec.precision = CountIsStar(i);
/* FP:lib.rs-0651 */             } else {
/* FP:lib.rs-0652 */                 spec.precision = self.count();
/* FP:lib.rs-0653 */             }
/* FP:lib.rs-0654 */             spec.precision_span =
/* FP:lib.rs-0655 */                 Some(range.start..self.input_vec_index2range(self.input_vec_index).start);
/* FP:lib.rs-0656 */         }
/* FP:lib.rs-0657 */ 
/* FP:lib.rs-0658 */         let start_idx = self.input_vec_index;
/* FP:lib.rs-0659 */         // Optional radix followed by the actual format specifier
/* FP:lib.rs-0660 */         if self.consume('x') {
/* FP:lib.rs-0661 */             if self.consume('?') {
/* FP:lib.rs-0662 */                 spec.debug_hex = Some(DebugHex::Lower);
/* FP:lib.rs-0663 */                 spec.ty = "?";
/* FP:lib.rs-0664 */             } else {
/* FP:lib.rs-0665 */                 spec.ty = "x";
/* FP:lib.rs-0666 */             }
/* FP:lib.rs-0667 */         } else if self.consume('X') {
/* FP:lib.rs-0668 */             if self.consume('?') {
/* FP:lib.rs-0669 */                 spec.debug_hex = Some(DebugHex::Upper);
/* FP:lib.rs-0670 */                 spec.ty = "?";
/* FP:lib.rs-0671 */             } else {
/* FP:lib.rs-0672 */                 spec.ty = "X";
/* FP:lib.rs-0673 */             }
/* FP:lib.rs-0674 */         } else if let Some((range, _)) = self.consume_pos('?') {
/* FP:lib.rs-0675 */             spec.ty = "?";
/* FP:lib.rs-0676 */             if let Some((r, _, c @ ('#' | 'x' | 'X'))) = self.peek() {
/* FP:lib.rs-0677 */                 self.errors.insert(
/* FP:lib.rs-0678 */                     0,
/* FP:lib.rs-0679 */                     ParseError {
/* FP:lib.rs-0680 */                         description: format!("expected `}}`, found `{c}`"),
/* FP:lib.rs-0681 */                         note: None,
/* FP:lib.rs-0682 */                         label: "expected `'}'`".into(),
/* FP:lib.rs-0683 */                         span: r.clone(),
/* FP:lib.rs-0684 */                         secondary_label: None,
/* FP:lib.rs-0685 */                         suggestion: Suggestion::ReorderFormatParameter(
/* FP:lib.rs-0686 */                             range.start..r.end,
/* FP:lib.rs-0687 */                             format!("{c}?"),
/* FP:lib.rs-0688 */                         ),
/* FP:lib.rs-0689 */                     },
/* FP:lib.rs-0690 */                 );
/* FP:lib.rs-0691 */             }
/* FP:lib.rs-0692 */         } else {
/* FP:lib.rs-0693 */             spec.ty = self.word();
/* FP:lib.rs-0694 */             if !spec.ty.is_empty() {
/* FP:lib.rs-0695 */                 let start = self.input_vec_index2range(start_idx).start;
/* FP:lib.rs-0696 */                 let end = self.input_vec_index2range(self.input_vec_index).start;
/* FP:lib.rs-0697 */                 spec.ty_span = Some(start..end);
/* FP:lib.rs-0698 */             }
/* FP:lib.rs-0699 */         }
/* FP:lib.rs-0700 */         spec
/* FP:lib.rs-0701 */     }
/* FP:lib.rs-0702 */ 
/* FP:lib.rs-0703 */     /// Parses an inline assembly template modifier at the current position, returning the modifier
/* FP:lib.rs-0704 */     /// in the `ty` field of the `FormatSpec` struct.
/* FP:lib.rs-0705 */     fn inline_asm(&mut self) -> FormatSpec<'input> {
/* FP:lib.rs-0706 */         let mut spec = FormatSpec::default();
/* FP:lib.rs-0707 */ 
/* FP:lib.rs-0708 */         if !self.consume(':') {
/* FP:lib.rs-0709 */             return spec;
/* FP:lib.rs-0710 */         }
/* FP:lib.rs-0711 */ 
/* FP:lib.rs-0712 */         let start_idx = self.input_vec_index;
/* FP:lib.rs-0713 */         spec.ty = self.word();
/* FP:lib.rs-0714 */         if !spec.ty.is_empty() {
/* FP:lib.rs-0715 */             let start = self.input_vec_index2range(start_idx).start;
/* FP:lib.rs-0716 */             let end = self.input_vec_index2range(self.input_vec_index).start;
/* FP:lib.rs-0717 */             spec.ty_span = Some(start..end);
/* FP:lib.rs-0718 */         }
/* FP:lib.rs-0719 */ 
/* FP:lib.rs-0720 */         spec
/* FP:lib.rs-0721 */     }
/* FP:lib.rs-0722 */ 
/* FP:lib.rs-0723 */     /// Always returns an empty `FormatSpec`
/* FP:lib.rs-0724 */     fn diagnostic(&mut self) -> FormatSpec<'input> {
/* FP:lib.rs-0725 */         let mut spec = FormatSpec::default();
/* FP:lib.rs-0726 */ 
/* FP:lib.rs-0727 */         let Some((Range { start, .. }, start_idx)) = self.consume_pos(':') else {
/* FP:lib.rs-0728 */             return spec;
/* FP:lib.rs-0729 */         };
/* FP:lib.rs-0730 */ 
/* FP:lib.rs-0731 */         spec.ty = self.string(start_idx);
/* FP:lib.rs-0732 */         spec.ty_span = {
/* FP:lib.rs-0733 */             let end = self.input_vec_index2range(self.input_vec_index).start;
/* FP:lib.rs-0734 */             Some(start..end)
/* FP:lib.rs-0735 */         };
/* FP:lib.rs-0736 */         spec
/* FP:lib.rs-0737 */     }
/* FP:lib.rs-0738 */ 
/* FP:lib.rs-0739 */     /// Parses a `Count` parameter at the current position. This does not check
/* FP:lib.rs-0740 */     /// for 'CountIsNextParam' because that is only used in precision, not
/* FP:lib.rs-0741 */     /// width.
/* FP:lib.rs-0742 */     fn count(&mut self) -> Count<'input> {
/* FP:lib.rs-0743 */         if let Some(i) = self.integer() {
/* FP:lib.rs-0744 */             if self.consume('$') { CountIsParam(i.into()) } else { CountIs(i) }
/* FP:lib.rs-0745 */         } else {
/* FP:lib.rs-0746 */             let start_idx = self.input_vec_index;
/* FP:lib.rs-0747 */             let word = self.word();
/* FP:lib.rs-0748 */             if word.is_empty() {
/* FP:lib.rs-0749 */                 CountImplied
/* FP:lib.rs-0750 */             } else if let Some((r, _)) = self.consume_pos('$') {
/* FP:lib.rs-0751 */                 CountIsName(word, self.input_vec_index2range(start_idx).start..r.start)
/* FP:lib.rs-0752 */             } else {
/* FP:lib.rs-0753 */                 self.input_vec_index = start_idx;
/* FP:lib.rs-0754 */                 CountImplied
/* FP:lib.rs-0755 */             }
/* FP:lib.rs-0756 */         }
/* FP:lib.rs-0757 */     }
/* FP:lib.rs-0758 */ 
/* FP:lib.rs-0759 */     /// Parses a word starting at the current position. A word is the same as a
/* FP:lib.rs-0760 */     /// Rust identifier, except that it can't start with `_` character.
/* FP:lib.rs-0761 */     fn word(&mut self) -> &'input str {
/* FP:lib.rs-0762 */         let index = self.input_vec_index;
/* FP:lib.rs-0763 */         match self.peek() {
/* FP:lib.rs-0764 */             Some((ref r, i, c)) if rustc_lexer::is_id_start(c) => {
/* FP:lib.rs-0765 */                 self.input_vec_index += 1;
/* FP:lib.rs-0766 */                 (r.start, i)
/* FP:lib.rs-0767 */             }
/* FP:lib.rs-0768 */             _ => {
/* FP:lib.rs-0769 */                 return "";
/* FP:lib.rs-0770 */             }
/* FP:lib.rs-0771 */         };
/* FP:lib.rs-0772 */         let (err_end, end): (usize, usize) = loop {
/* FP:lib.rs-0773 */             if let Some((ref r, i, c)) = self.peek() {
/* FP:lib.rs-0774 */                 if rustc_lexer::is_id_continue(c) {
/* FP:lib.rs-0775 */                     self.input_vec_index += 1;
/* FP:lib.rs-0776 */                 } else {
/* FP:lib.rs-0777 */                     break (r.start, i);
/* FP:lib.rs-0778 */                 }
/* FP:lib.rs-0779 */             } else {
/* FP:lib.rs-0780 */                 break (self.end_of_snippet, self.input.len());
/* FP:lib.rs-0781 */             }
/* FP:lib.rs-0782 */         };
/* FP:lib.rs-0783 */ 
/* FP:lib.rs-0784 */         let word = &self.input[self.input_vec_index2pos(index)..end];
/* FP:lib.rs-0785 */         if word == "_" {
/* FP:lib.rs-0786 */             self.errors.push(ParseError {
/* FP:lib.rs-0787 */                 description: "invalid argument name `_`".into(),
/* FP:lib.rs-0788 */                 note: Some("argument name cannot be a single underscore".into()),
/* FP:lib.rs-0789 */                 label: "invalid argument name".into(),
/* FP:lib.rs-0790 */                 span: self.input_vec_index2range(index).start..err_end,
/* FP:lib.rs-0791 */                 secondary_label: None,
/* FP:lib.rs-0792 */                 suggestion: Suggestion::None,
/* FP:lib.rs-0793 */             });
/* FP:lib.rs-0794 */         }
/* FP:lib.rs-0795 */         word
/* FP:lib.rs-0796 */     }
/* FP:lib.rs-0797 */ 
/* FP:lib.rs-0798 */     fn integer(&mut self) -> Option<u16> {
/* FP:lib.rs-0799 */         let mut cur: u16 = 0;
/* FP:lib.rs-0800 */         let mut found = false;
/* FP:lib.rs-0801 */         let mut overflow = false;
/* FP:lib.rs-0802 */         let start_index = self.input_vec_index;
/* FP:lib.rs-0803 */         while let Some((_, _, c)) = self.peek() {
/* FP:lib.rs-0804 */             if let Some(i) = c.to_digit(10) {
/* FP:lib.rs-0805 */                 self.input_vec_index += 1;
/* FP:lib.rs-0806 */                 let (tmp, mul_overflow) = cur.overflowing_mul(10);
/* FP:lib.rs-0807 */                 let (tmp, add_overflow) = tmp.overflowing_add(i as u16);
/* FP:lib.rs-0808 */                 if mul_overflow || add_overflow {
/* FP:lib.rs-0809 */                     overflow = true;
/* FP:lib.rs-0810 */                 }
/* FP:lib.rs-0811 */                 cur = tmp;
/* FP:lib.rs-0812 */                 found = true;
/* FP:lib.rs-0813 */             } else {
/* FP:lib.rs-0814 */                 break;
/* FP:lib.rs-0815 */             }
/* FP:lib.rs-0816 */         }
/* FP:lib.rs-0817 */ 
/* FP:lib.rs-0818 */         if overflow {
/* FP:lib.rs-0819 */             let overflowed_int = &self.input[self.input_vec_index2pos(start_index)
/* FP:lib.rs-0820 */                 ..self.input_vec_index2pos(self.input_vec_index)];
/* FP:lib.rs-0821 */             self.errors.push(ParseError {
/* FP:lib.rs-0822 */                 description: format!(
/* FP:lib.rs-0823 */                     "integer `{}` does not fit into the type `u16` whose range is `0..={}`",
/* FP:lib.rs-0824 */                     overflowed_int,
/* FP:lib.rs-0825 */                     u16::MAX
/* FP:lib.rs-0826 */                 ),
/* FP:lib.rs-0827 */                 note: None,
/* FP:lib.rs-0828 */                 label: "integer out of range for `u16`".into(),
/* FP:lib.rs-0829 */                 span: self.input_vec_index2range(start_index).start
/* FP:lib.rs-0830 */                     ..self.input_vec_index2range(self.input_vec_index).end,
/* FP:lib.rs-0831 */                 secondary_label: None,
/* FP:lib.rs-0832 */                 suggestion: Suggestion::None,
/* FP:lib.rs-0833 */             });
/* FP:lib.rs-0834 */         }
/* FP:lib.rs-0835 */ 
/* FP:lib.rs-0836 */         found.then_some(cur)
/* FP:lib.rs-0837 */     }
/* FP:lib.rs-0838 */ 
/* FP:lib.rs-0839 */     fn suggest_format_debug(&mut self) {
/* FP:lib.rs-0840 */         if let (Some((range, _)), Some(_)) = (self.consume_pos('?'), self.consume_pos(':')) {
/* FP:lib.rs-0841 */             let word = self.word();
/* FP:lib.rs-0842 */             self.errors.insert(
/* FP:lib.rs-0843 */                 0,
/* FP:lib.rs-0844 */                 ParseError {
/* FP:lib.rs-0845 */                     description: "expected format parameter to occur after `:`".to_owned(),
/* FP:lib.rs-0846 */                     note: Some(format!("`?` comes after `:`, try `{}:{}` instead", word, "?")),
/* FP:lib.rs-0847 */                     label: "expected `?` to occur after `:`".to_owned(),
/* FP:lib.rs-0848 */                     span: range,
/* FP:lib.rs-0849 */                     secondary_label: None,
/* FP:lib.rs-0850 */                     suggestion: Suggestion::None,
/* FP:lib.rs-0851 */                 },
/* FP:lib.rs-0852 */             );
/* FP:lib.rs-0853 */         }
/* FP:lib.rs-0854 */     }
/* FP:lib.rs-0855 */ 
/* FP:lib.rs-0856 */     fn suggest_format_align(&mut self, alignment: char) {
/* FP:lib.rs-0857 */         if let Some((range, _)) = self.consume_pos(alignment) {
/* FP:lib.rs-0858 */             self.errors.insert(
/* FP:lib.rs-0859 */                 0,
/* FP:lib.rs-0860 */                 ParseError {
/* FP:lib.rs-0861 */                     description:
/* FP:lib.rs-0862 */                         "expected alignment specifier after `:` in format string; example: `{:>?}`"
/* FP:lib.rs-0863 */                             .to_owned(),
/* FP:lib.rs-0864 */                     note: None,
/* FP:lib.rs-0865 */                     label: format!("expected `{}` to occur after `:`", alignment),
/* FP:lib.rs-0866 */                     span: range,
/* FP:lib.rs-0867 */                     secondary_label: None,
/* FP:lib.rs-0868 */                     suggestion: Suggestion::None,
/* FP:lib.rs-0869 */                 },
/* FP:lib.rs-0870 */             );
/* FP:lib.rs-0871 */         }
/* FP:lib.rs-0872 */     }
/* FP:lib.rs-0873 */ 
/* FP:lib.rs-0874 */     fn suggest_positional_arg_instead_of_captured_arg(&mut self, arg: &Argument<'_>) {
/* FP:lib.rs-0875 */         // If the argument is not an identifier, it is not a field access.
/* FP:lib.rs-0876 */         if !arg.is_identifier() {
/* FP:lib.rs-0877 */             return;
/* FP:lib.rs-0878 */         }
/* FP:lib.rs-0879 */ 
/* FP:lib.rs-0880 */         if let Some((_range, _pos)) = self.consume_pos('.') {
/* FP:lib.rs-0881 */             let field = self.argument();
/* FP:lib.rs-0882 */             // We can only parse simple `foo.bar` field access or `foo.0` tuple index access, any
/* FP:lib.rs-0883 */             // deeper nesting, or another type of expression, like method calls, are not supported
/* FP:lib.rs-0884 */             if !self.consume('}') {
/* FP:lib.rs-0885 */                 return;
/* FP:lib.rs-0886 */             }
/* FP:lib.rs-0887 */             if let ArgumentNamed(_) = arg.position {
/* FP:lib.rs-0888 */                 match field.position {
/* FP:lib.rs-0889 */                     ArgumentNamed(_) => {
/* FP:lib.rs-0890 */                         self.errors.insert(
/* FP:lib.rs-0891 */                             0,
/* FP:lib.rs-0892 */                             ParseError {
/* FP:lib.rs-0893 */                                 description: "field access isn't supported".to_string(),
/* FP:lib.rs-0894 */                                 note: None,
/* FP:lib.rs-0895 */                                 label: "not supported".to_string(),
/* FP:lib.rs-0896 */                                 span: arg.position_span.start..field.position_span.end,
/* FP:lib.rs-0897 */                                 secondary_label: None,
/* FP:lib.rs-0898 */                                 suggestion: Suggestion::UsePositional,
/* FP:lib.rs-0899 */                             },
/* FP:lib.rs-0900 */                         );
/* FP:lib.rs-0901 */                     }
/* FP:lib.rs-0902 */                     ArgumentIs(_) => {
/* FP:lib.rs-0903 */                         self.errors.insert(
/* FP:lib.rs-0904 */                             0,
/* FP:lib.rs-0905 */                             ParseError {
/* FP:lib.rs-0906 */                                 description: "tuple index access isn't supported".to_string(),
/* FP:lib.rs-0907 */                                 note: None,
/* FP:lib.rs-0908 */                                 label: "not supported".to_string(),
/* FP:lib.rs-0909 */                                 span: arg.position_span.start..field.position_span.end,
/* FP:lib.rs-0910 */                                 secondary_label: None,
/* FP:lib.rs-0911 */                                 suggestion: Suggestion::UsePositional,
/* FP:lib.rs-0912 */                             },
/* FP:lib.rs-0913 */                         );
/* FP:lib.rs-0914 */                     }
/* FP:lib.rs-0915 */                     _ => {}
/* FP:lib.rs-0916 */                 };
/* FP:lib.rs-0917 */             }
/* FP:lib.rs-0918 */         }
/* FP:lib.rs-0919 */     }
/* FP:lib.rs-0920 */ }
/* FP:lib.rs-0921 */ 
/* FP:lib.rs-0922 */ // Assert a reasonable size for `Piece`
/* FP:lib.rs-0923 */ #[cfg(all(test, target_pointer_width = "64"))]
/* FP:lib.rs-0924 */ crate::rustc_index::static_assert_size!(Piece<'_>, 16);
/* FP:lib.rs-0925 */ 
/* FP:lib.rs-0926 */ #[cfg(test)]