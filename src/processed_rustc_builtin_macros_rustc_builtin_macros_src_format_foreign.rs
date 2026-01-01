/* FP:format_foreign.rs-0001 */ pub(crate) mod printf {
/* FP:format_foreign.rs-0002 */     use crate::rustc_complete::InnerSpan;
/* FP:format_foreign.rs-0003 */ 
/* FP:format_foreign.rs-0004 */     use super::strcursor::StrCursor as Cur;
/* FP:format_foreign.rs-0005 */ 
/* FP:format_foreign.rs-0006 */     /// Represents a single `printf`-style substitution.
/* FP:format_foreign.rs-0007 */     #[derive(Clone, PartialEq, Debug)]
/* FP:format_foreign.rs-0008 */     pub(crate) enum Substitution<'a> {
/* FP:format_foreign.rs-0009 */         /// A formatted output substitution with its internal byte offset.
/* FP:format_foreign.rs-0010 */         Format(Format<'a>),
/* FP:format_foreign.rs-0011 */         /// A literal `%%` escape, with its start and end indices.
/* FP:format_foreign.rs-0012 */         Escape((usize, usize)),
/* FP:format_foreign.rs-0013 */     }
/* FP:format_foreign.rs-0014 */ 
/* FP:format_foreign.rs-0015 */     impl ToString for Substitution<'_> {
/* FP:format_foreign.rs-0016 */         fn to_string(&self) -> String {
/* FP:format_foreign.rs-0017 */             match self {
/* FP:format_foreign.rs-0018 */                 Substitution::Format(fmt) => fmt.span.into(),
/* FP:format_foreign.rs-0019 */                 Substitution::Escape(_) => "%%".into(),
/* FP:format_foreign.rs-0020 */             }
/* FP:format_foreign.rs-0021 */         }
/* FP:format_foreign.rs-0022 */     }
/* FP:format_foreign.rs-0023 */ 
/* FP:format_foreign.rs-0024 */     impl Substitution<'_> {
/* FP:format_foreign.rs-0025 */         pub(crate) fn position(&self) -> InnerSpan {
/* FP:format_foreign.rs-0026 */             match self {
/* FP:format_foreign.rs-0027 */                 Substitution::Format(fmt) => fmt.position,
/* FP:format_foreign.rs-0028 */                 &Substitution::Escape((start, end)) => InnerSpan::new(start, end),
/* FP:format_foreign.rs-0029 */             }
/* FP:format_foreign.rs-0030 */         }
/* FP:format_foreign.rs-0031 */ 
/* FP:format_foreign.rs-0032 */         pub(crate) fn set_position(&mut self, start: usize, end: usize) {
/* FP:format_foreign.rs-0033 */             match self {
/* FP:format_foreign.rs-0034 */                 Substitution::Format(fmt) => fmt.position = InnerSpan::new(start, end),
/* FP:format_foreign.rs-0035 */                 Substitution::Escape(pos) => *pos = (start, end),
/* FP:format_foreign.rs-0036 */             }
/* FP:format_foreign.rs-0037 */         }
/* FP:format_foreign.rs-0038 */ 
/* FP:format_foreign.rs-0039 */         /// Translate this substitution into an equivalent Rust formatting directive.
/* FP:format_foreign.rs-0040 */         ///
/* FP:format_foreign.rs-0041 */         /// This ignores cases where the substitution does not have an exact equivalent, or where
/* FP:format_foreign.rs-0042 */         /// the substitution would be unnecessary.
/* FP:format_foreign.rs-0043 */         pub(crate) fn translate(&self) -> Result<String, Option<String>> {
/* FP:format_foreign.rs-0044 */             match self {
/* FP:format_foreign.rs-0045 */                 Substitution::Format(fmt) => fmt.translate(),
/* FP:format_foreign.rs-0046 */                 Substitution::Escape(_) => Err(None),
/* FP:format_foreign.rs-0047 */             }
/* FP:format_foreign.rs-0048 */         }
/* FP:format_foreign.rs-0049 */     }
/* FP:format_foreign.rs-0050 */ 
/* FP:format_foreign.rs-0051 */     #[derive(Clone, PartialEq, Debug)]
/* FP:format_foreign.rs-0052 */     /// A single `printf`-style formatting directive.
/* FP:format_foreign.rs-0053 */     pub(crate) struct Format<'a> {
/* FP:format_foreign.rs-0054 */         /// The entire original formatting directive.
/* FP:format_foreign.rs-0055 */         span: &'a str,
/* FP:format_foreign.rs-0056 */         /// The (1-based) parameter to be converted.
/* FP:format_foreign.rs-0057 */         parameter: Option<u16>,
/* FP:format_foreign.rs-0058 */         /// Formatting flags.
/* FP:format_foreign.rs-0059 */         flags: &'a str,
/* FP:format_foreign.rs-0060 */         /// Minimum width of the output.
/* FP:format_foreign.rs-0061 */         width: Option<Num>,
/* FP:format_foreign.rs-0062 */         /// Precision of the conversion.
/* FP:format_foreign.rs-0063 */         precision: Option<Num>,
/* FP:format_foreign.rs-0064 */         /// Length modifier for the conversion.
/* FP:format_foreign.rs-0065 */         length: Option<&'a str>,
/* FP:format_foreign.rs-0066 */         /// Type of parameter being converted.
/* FP:format_foreign.rs-0067 */         type_: &'a str,
/* FP:format_foreign.rs-0068 */         /// Byte offset for the start and end of this formatting directive.
/* FP:format_foreign.rs-0069 */         position: InnerSpan,
/* FP:format_foreign.rs-0070 */     }
/* FP:format_foreign.rs-0071 */ 
/* FP:format_foreign.rs-0072 */     impl Format<'_> {
/* FP:format_foreign.rs-0073 */         /// Translate this directive into an equivalent Rust formatting directive.
/* FP:format_foreign.rs-0074 */         ///
/* FP:format_foreign.rs-0075 */         /// Returns `Err` in cases where the `printf` directive does not have an exact Rust
/* FP:format_foreign.rs-0076 */         /// equivalent, rather than guessing.
/* FP:format_foreign.rs-0077 */         pub(crate) fn translate(&self) -> Result<String, Option<String>> {
/* FP:format_foreign.rs-0078 */             use std::fmt::Write;
/* FP:format_foreign.rs-0079 */ 
/* FP:format_foreign.rs-0080 */             let (c_alt, c_zero, c_left, c_plus) = {
/* FP:format_foreign.rs-0081 */                 let mut c_alt = false;
/* FP:format_foreign.rs-0082 */                 let mut c_zero = false;
/* FP:format_foreign.rs-0083 */                 let mut c_left = false;
/* FP:format_foreign.rs-0084 */                 let mut c_plus = false;
/* FP:format_foreign.rs-0085 */                 for c in self.flags.chars() {
/* FP:format_foreign.rs-0086 */                     match c {
/* FP:format_foreign.rs-0087 */                         '#' => c_alt = true,
/* FP:format_foreign.rs-0088 */                         '0' => c_zero = true,
/* FP:format_foreign.rs-0089 */                         '-' => c_left = true,
/* FP:format_foreign.rs-0090 */                         '+' => c_plus = true,
/* FP:format_foreign.rs-0091 */                         _ => {
/* FP:format_foreign.rs-0092 */                             return Err(Some(format!("the flag `{c}` is unknown or unsupported")));
/* FP:format_foreign.rs-0093 */                         }
/* FP:format_foreign.rs-0094 */                     }
/* FP:format_foreign.rs-0095 */                 }
/* FP:format_foreign.rs-0096 */                 (c_alt, c_zero, c_left, c_plus)
/* FP:format_foreign.rs-0097 */             };
/* FP:format_foreign.rs-0098 */ 
/* FP:format_foreign.rs-0099 */             // Has a special form in Rust for numbers.
/* FP:format_foreign.rs-0100 */             let fill = c_zero.then_some("0");
/* FP:format_foreign.rs-0101 */ 
/* FP:format_foreign.rs-0102 */             let align = c_left.then_some("<");
/* FP:format_foreign.rs-0103 */ 
/* FP:format_foreign.rs-0104 */             // Rust doesn't have an equivalent to the `' '` flag.
/* FP:format_foreign.rs-0105 */             let sign = c_plus.then_some("+");
/* FP:format_foreign.rs-0106 */ 
/* FP:format_foreign.rs-0107 */             // Not *quite* the same, depending on the type...
/* FP:format_foreign.rs-0108 */             let alt = c_alt;
/* FP:format_foreign.rs-0109 */ 
/* FP:format_foreign.rs-0110 */             let width = match self.width {
/* FP:format_foreign.rs-0111 */                 Some(Num::Next) => {
/* FP:format_foreign.rs-0112 */                     // NOTE: Rust doesn't support this.
/* FP:format_foreign.rs-0113 */                     return Err(Some(
/* FP:format_foreign.rs-0114 */                         "you have to use a positional or named parameter for the width".to_string(),
/* FP:format_foreign.rs-0115 */                     ));
/* FP:format_foreign.rs-0116 */                 }
/* FP:format_foreign.rs-0117 */                 w @ Some(Num::Arg(_)) => w,
/* FP:format_foreign.rs-0118 */                 w @ Some(Num::Num(_)) => w,
/* FP:format_foreign.rs-0119 */                 None => None,
/* FP:format_foreign.rs-0120 */             };
/* FP:format_foreign.rs-0121 */ 
/* FP:format_foreign.rs-0122 */             let precision = self.precision;
/* FP:format_foreign.rs-0123 */ 
/* FP:format_foreign.rs-0124 */             // NOTE: although length *can* have an effect, we can't duplicate the effect in Rust, so
/* FP:format_foreign.rs-0125 */             // we just ignore it.
/* FP:format_foreign.rs-0126 */ 
/* FP:format_foreign.rs-0127 */             let (type_, use_zero_fill, is_int) = match self.type_ {
/* FP:format_foreign.rs-0128 */                 "d" | "i" | "u" => (None, true, true),
/* FP:format_foreign.rs-0129 */                 "f" | "F" => (None, false, false),
/* FP:format_foreign.rs-0130 */                 "s" | "c" => (None, false, false),
/* FP:format_foreign.rs-0131 */                 "e" | "E" => (Some(self.type_), true, false),
/* FP:format_foreign.rs-0132 */                 "x" | "X" | "o" => (Some(self.type_), true, true),
/* FP:format_foreign.rs-0133 */                 "p" => (Some(self.type_), false, true),
/* FP:format_foreign.rs-0134 */                 "g" => (Some("e"), true, false),
/* FP:format_foreign.rs-0135 */                 "G" => (Some("E"), true, false),
/* FP:format_foreign.rs-0136 */                 _ => {
/* FP:format_foreign.rs-0137 */                     return Err(Some(format!(
/* FP:format_foreign.rs-0138 */                         "the conversion specifier `{}` is unknown or unsupported",
/* FP:format_foreign.rs-0139 */                         self.type_
/* FP:format_foreign.rs-0140 */                     )));
/* FP:format_foreign.rs-0141 */                 }
/* FP:format_foreign.rs-0142 */             };
/* FP:format_foreign.rs-0143 */ 
/* FP:format_foreign.rs-0144 */             let (fill, width, precision) = match (is_int, width, precision) {
/* FP:format_foreign.rs-0145 */                 (true, Some(_), Some(_)) => {
/* FP:format_foreign.rs-0146 */                     // Rust can't duplicate this insanity.
/* FP:format_foreign.rs-0147 */                     return Err(Some(
/* FP:format_foreign.rs-0148 */                         "width and precision cannot both be specified for integer conversions"
/* FP:format_foreign.rs-0149 */                             .to_string(),
/* FP:format_foreign.rs-0150 */                     ));
/* FP:format_foreign.rs-0151 */                 }
/* FP:format_foreign.rs-0152 */                 (true, None, Some(p)) => (Some("0"), Some(p), None),
/* FP:format_foreign.rs-0153 */                 (true, w, None) => (fill, w, None),
/* FP:format_foreign.rs-0154 */                 (false, w, p) => (fill, w, p),
/* FP:format_foreign.rs-0155 */             };
/* FP:format_foreign.rs-0156 */ 
/* FP:format_foreign.rs-0157 */             let align = match (self.type_, width.is_some(), align.is_some()) {
/* FP:format_foreign.rs-0158 */                 ("s", true, false) => Some(">"),
/* FP:format_foreign.rs-0159 */                 _ => align,
/* FP:format_foreign.rs-0160 */             };
/* FP:format_foreign.rs-0161 */ 
/* FP:format_foreign.rs-0162 */             let (fill, zero_fill) = match (fill, use_zero_fill) {
/* FP:format_foreign.rs-0163 */                 (Some("0"), true) => (None, true),
/* FP:format_foreign.rs-0164 */                 (fill, _) => (fill, false),
/* FP:format_foreign.rs-0165 */             };
/* FP:format_foreign.rs-0166 */ 
/* FP:format_foreign.rs-0167 */             let alt = match type_ {
/* FP:format_foreign.rs-0168 */                 Some("x" | "X") => alt,
/* FP:format_foreign.rs-0169 */                 _ => false,
/* FP:format_foreign.rs-0170 */             };
/* FP:format_foreign.rs-0171 */ 
/* FP:format_foreign.rs-0172 */             let has_options = fill.is_some()
/* FP:format_foreign.rs-0173 */                 || align.is_some()
/* FP:format_foreign.rs-0174 */                 || sign.is_some()
/* FP:format_foreign.rs-0175 */                 || alt
/* FP:format_foreign.rs-0176 */                 || zero_fill
/* FP:format_foreign.rs-0177 */                 || width.is_some()
/* FP:format_foreign.rs-0178 */                 || precision.is_some()
/* FP:format_foreign.rs-0179 */                 || type_.is_some();
/* FP:format_foreign.rs-0180 */ 
/* FP:format_foreign.rs-0181 */             // Initialise with a rough guess.
/* FP:format_foreign.rs-0182 */             let cap = self.span.len() + if has_options { 2 } else { 0 };
/* FP:format_foreign.rs-0183 */             let mut s = String::with_capacity(cap);
/* FP:format_foreign.rs-0184 */ 
/* FP:format_foreign.rs-0185 */             s.push('{');
/* FP:format_foreign.rs-0186 */ 
/* FP:format_foreign.rs-0187 */             if let Some(arg) = self.parameter {
/* FP:format_foreign.rs-0188 */                 match write!(
/* FP:format_foreign.rs-0189 */                     s,
/* FP:format_foreign.rs-0190 */                     "{}",
/* FP:format_foreign.rs-0191 */                     match arg.checked_sub(1) {
/* FP:format_foreign.rs-0192 */                         Some(a) => a,
/* FP:format_foreign.rs-0193 */                         None => return Err(None),
/* FP:format_foreign.rs-0194 */                     }
/* FP:format_foreign.rs-0195 */                 ) {
/* FP:format_foreign.rs-0196 */                     Err(_) => return Err(None),
/* FP:format_foreign.rs-0197 */                     _ => {}
/* FP:format_foreign.rs-0198 */                 }
/* FP:format_foreign.rs-0199 */             }
/* FP:format_foreign.rs-0200 */ 
/* FP:format_foreign.rs-0201 */             if has_options {
/* FP:format_foreign.rs-0202 */                 s.push(':');
/* FP:format_foreign.rs-0203 */ 
/* FP:format_foreign.rs-0204 */                 let align = if let Some(fill) = fill {
/* FP:format_foreign.rs-0205 */                     s.push_str(fill);
/* FP:format_foreign.rs-0206 */                     align.or(Some(">"))
/* FP:format_foreign.rs-0207 */                 } else {
/* FP:format_foreign.rs-0208 */                     align
/* FP:format_foreign.rs-0209 */                 };
/* FP:format_foreign.rs-0210 */ 
/* FP:format_foreign.rs-0211 */                 if let Some(align) = align {
/* FP:format_foreign.rs-0212 */                     s.push_str(align);
/* FP:format_foreign.rs-0213 */                 }
/* FP:format_foreign.rs-0214 */ 
/* FP:format_foreign.rs-0215 */                 if let Some(sign) = sign {
/* FP:format_foreign.rs-0216 */                     s.push_str(sign);
/* FP:format_foreign.rs-0217 */                 }
/* FP:format_foreign.rs-0218 */ 
/* FP:format_foreign.rs-0219 */                 if alt {
/* FP:format_foreign.rs-0220 */                     s.push('#');
/* FP:format_foreign.rs-0221 */                 }
/* FP:format_foreign.rs-0222 */ 
/* FP:format_foreign.rs-0223 */                 if zero_fill {
/* FP:format_foreign.rs-0224 */                     s.push('0');
/* FP:format_foreign.rs-0225 */                 }
/* FP:format_foreign.rs-0226 */ 
/* FP:format_foreign.rs-0227 */                 if let Some(width) = width {
/* FP:format_foreign.rs-0228 */                     match width.translate(&mut s) {
/* FP:format_foreign.rs-0229 */                         Err(_) => return Err(None),
/* FP:format_foreign.rs-0230 */                         _ => {}
/* FP:format_foreign.rs-0231 */                     }
/* FP:format_foreign.rs-0232 */                 }
/* FP:format_foreign.rs-0233 */ 
/* FP:format_foreign.rs-0234 */                 if let Some(precision) = precision {
/* FP:format_foreign.rs-0235 */                     s.push('.');
/* FP:format_foreign.rs-0236 */                     match precision.translate(&mut s) {
/* FP:format_foreign.rs-0237 */                         Err(_) => return Err(None),
/* FP:format_foreign.rs-0238 */                         _ => {}
/* FP:format_foreign.rs-0239 */                     }
/* FP:format_foreign.rs-0240 */                 }
/* FP:format_foreign.rs-0241 */ 
/* FP:format_foreign.rs-0242 */                 if let Some(type_) = type_ {
/* FP:format_foreign.rs-0243 */                     s.push_str(type_);
/* FP:format_foreign.rs-0244 */                 }
/* FP:format_foreign.rs-0245 */             }
/* FP:format_foreign.rs-0246 */ 
/* FP:format_foreign.rs-0247 */             s.push('}');
/* FP:format_foreign.rs-0248 */             Ok(s)
/* FP:format_foreign.rs-0249 */         }
/* FP:format_foreign.rs-0250 */     }
/* FP:format_foreign.rs-0251 */ 
/* FP:format_foreign.rs-0252 */     /// A general number used in a `printf` formatting directive.
/* FP:format_foreign.rs-0253 */     #[derive(Copy, Clone, PartialEq, Debug)]
/* FP:format_foreign.rs-0254 */     enum Num {
/* FP:format_foreign.rs-0255 */         // The range of these values is technically bounded by `NL_ARGMAX`... but, at least for GNU
/* FP:format_foreign.rs-0256 */         // libc, it apparently has no real fixed limit. A `u16` is used here on the basis that it
/* FP:format_foreign.rs-0257 */         // is *vanishingly* unlikely that *anyone* is going to try formatting something wider, or
/* FP:format_foreign.rs-0258 */         // with more precision, than 32 thousand positions which is so wide it couldn't possibly fit
/* FP:format_foreign.rs-0259 */         // on a screen.
/* FP:format_foreign.rs-0260 */         /// A specific, fixed value.
/* FP:format_foreign.rs-0261 */         Num(u16),
/* FP:format_foreign.rs-0262 */         /// The value is derived from a positional argument.
/* FP:format_foreign.rs-0263 */         Arg(u16),
/* FP:format_foreign.rs-0264 */         /// The value is derived from the "next" unconverted argument.
/* FP:format_foreign.rs-0265 */         Next,
/* FP:format_foreign.rs-0266 */     }
/* FP:format_foreign.rs-0267 */ 
/* FP:format_foreign.rs-0268 */     impl Num {
/* FP:format_foreign.rs-0269 */         fn from_str(s: &str, arg: Option<&str>) -> Option<Self> {
/* FP:format_foreign.rs-0270 */             if let Some(arg) = arg {
/* FP:format_foreign.rs-0271 */                 arg.parse().ok().map(|arg| Num::Arg(arg))
/* FP:format_foreign.rs-0272 */             } else if s == "*" {
/* FP:format_foreign.rs-0273 */                 Some(Num::Next)
/* FP:format_foreign.rs-0274 */             } else {
/* FP:format_foreign.rs-0275 */                 s.parse().ok().map(|num| Num::Num(num))
/* FP:format_foreign.rs-0276 */             }
/* FP:format_foreign.rs-0277 */         }
/* FP:format_foreign.rs-0278 */ 
/* FP:format_foreign.rs-0279 */         fn translate(&self, s: &mut String) -> std::fmt::Result {
/* FP:format_foreign.rs-0280 */             use std::fmt::Write;
/* FP:format_foreign.rs-0281 */             match *self {
/* FP:format_foreign.rs-0282 */                 Num::Num(n) => write!(s, "{n}"),
/* FP:format_foreign.rs-0283 */                 Num::Arg(n) => {
/* FP:format_foreign.rs-0284 */                     let n = n.checked_sub(1).ok_or(std::fmt::Error)?;
/* FP:format_foreign.rs-0285 */                     write!(s, "{n}$")
/* FP:format_foreign.rs-0286 */                 }
/* FP:format_foreign.rs-0287 */                 Num::Next => write!(s, "*"),
/* FP:format_foreign.rs-0288 */             }
/* FP:format_foreign.rs-0289 */         }
/* FP:format_foreign.rs-0290 */     }
/* FP:format_foreign.rs-0291 */ 
/* FP:format_foreign.rs-0292 */     /// Returns an iterator over all substitutions in a given string.
/* FP:format_foreign.rs-0293 */     pub(crate) fn iter_subs(s: &str, start_pos: usize) -> Substitutions<'_> {
/* FP:format_foreign.rs-0294 */         Substitutions { s, pos: start_pos }
/* FP:format_foreign.rs-0295 */     }
/* FP:format_foreign.rs-0296 */ 
/* FP:format_foreign.rs-0297 */     /// Iterator over substitutions in a string.
/* FP:format_foreign.rs-0298 */     pub(crate) struct Substitutions<'a> {
/* FP:format_foreign.rs-0299 */         s: &'a str,
/* FP:format_foreign.rs-0300 */         pos: usize,
/* FP:format_foreign.rs-0301 */     }
/* FP:format_foreign.rs-0302 */ 
/* FP:format_foreign.rs-0303 */     impl<'a> Iterator for Substitutions<'a> {
/* FP:format_foreign.rs-0304 */         type Item = Substitution<'a>;
/* FP:format_foreign.rs-0305 */         fn next(&mut self) -> Option<Self::Item> {
/* FP:format_foreign.rs-0306 */             let (mut sub, tail) = parse_next_substitution(self.s)?;
/* FP:format_foreign.rs-0307 */             self.s = tail;
/* FP:format_foreign.rs-0308 */             let InnerSpan { start, end } = sub.position();
/* FP:format_foreign.rs-0309 */             sub.set_position(start + self.pos, end + self.pos);
/* FP:format_foreign.rs-0310 */             self.pos += end;
/* FP:format_foreign.rs-0311 */             Some(sub)
/* FP:format_foreign.rs-0312 */         }
/* FP:format_foreign.rs-0313 */ 
/* FP:format_foreign.rs-0314 */         fn size_hint(&self) -> (usize, Option<usize>) {
/* FP:format_foreign.rs-0315 */             // Substitutions are at least 2 characters long.
/* FP:format_foreign.rs-0316 */             (0, Some(self.s.len() / 2))
/* FP:format_foreign.rs-0317 */         }
/* FP:format_foreign.rs-0318 */     }
/* FP:format_foreign.rs-0319 */ 
/* FP:format_foreign.rs-0320 */     enum State {
/* FP:format_foreign.rs-0321 */         Start,
/* FP:format_foreign.rs-0322 */         Flags,
/* FP:format_foreign.rs-0323 */         Width,
/* FP:format_foreign.rs-0324 */         WidthArg,
/* FP:format_foreign.rs-0325 */         Prec,
/* FP:format_foreign.rs-0326 */         PrecInner,
/* FP:format_foreign.rs-0327 */         Length,
/* FP:format_foreign.rs-0328 */         Type,
/* FP:format_foreign.rs-0329 */     }
/* FP:format_foreign.rs-0330 */ 
/* FP:format_foreign.rs-0331 */     /// Parse the next substitution from the input string.
/* FP:format_foreign.rs-0332 */     fn parse_next_substitution(s: &str) -> Option<(Substitution<'_>, &str)> {
/* FP:format_foreign.rs-0333 */         use self::State::*;
/* FP:format_foreign.rs-0334 */ 
/* FP:format_foreign.rs-0335 */         let at = {
/* FP:format_foreign.rs-0336 */             let start = s.find('%')?;
/* FP:format_foreign.rs-0337 */             if let '%' = s[start + 1..].chars().next()? {
/* FP:format_foreign.rs-0338 */                 return Some((Substitution::Escape((start, start + 2)), &s[start + 2..]));
/* FP:format_foreign.rs-0339 */             }
/* FP:format_foreign.rs-0340 */ 
/* FP:format_foreign.rs-0341 */             Cur::new_at(s, start)
/* FP:format_foreign.rs-0342 */         };
/* FP:format_foreign.rs-0343 */ 
/* FP:format_foreign.rs-0344 */         // This is meant to be a translation of the following regex:
/* FP:format_foreign.rs-0345 */         //
/* FP:format_foreign.rs-0346 */         // ```regex
/* FP:format_foreign.rs-0347 */         // (?x)
/* FP:format_foreign.rs-0348 */         // ^ %
/* FP:format_foreign.rs-0349 */         // (?: (?Box<parameter> \d+) \$ )?
/* FP:format_foreign.rs-0350 */         // (?Box<flags> [-+ 0\#']* )
/* FP:format_foreign.rs-0351 */         // (?Box<width> \d+ | \* (?: (?Box<widtha> \d+) \$ )? )?
/* FP:format_foreign.rs-0352 */         // (?: \. (?Box<precision> \d+ | \* (?: (?Box<precisiona> \d+) \$ )? ) )?
/* FP:format_foreign.rs-0353 */         // (?Box<length>
/* FP:format_foreign.rs-0354 */         //     # Standard
/* FP:format_foreign.rs-0355 */         //     hh | h | ll | l | L | z | j | t
/* FP:format_foreign.rs-0356 */         //
/* FP:format_foreign.rs-0357 */         //     # Other
/* FP:format_foreign.rs-0358 */         //     | I32 | I64 | I | q
/* FP:format_foreign.rs-0359 */         // )?
/* FP:format_foreign.rs-0360 */         // (?Box<type> . )
/* FP:format_foreign.rs-0361 */         // ```
/* FP:format_foreign.rs-0362 */ 
/* FP:format_foreign.rs-0363 */         // Used to establish the full span at the end.
/* FP:format_foreign.rs-0364 */         let start = at;
/* FP:format_foreign.rs-0365 */         // The current position within the string.
/* FP:format_foreign.rs-0366 */         let mut at = at.at_next_cp()?;
/* FP:format_foreign.rs-0367 */         // `c` is the next codepoint, `next` is a cursor after it.
/* FP:format_foreign.rs-0368 */         let (mut c, mut next) = at.next_cp()?;
/* FP:format_foreign.rs-0369 */ 
/* FP:format_foreign.rs-0370 */         // Update `at`, `c`, and `next`, exiting if we're out of input.
/* FP:format_foreign.rs-0371 */         macro_rules! move_to {
/* FP:format_foreign.rs-0372 */             ($cur:expr) => {{
/* FP:format_foreign.rs-0373 */                 at = $cur;
/* FP:format_foreign.rs-0374 */                 let (c_, next_) = at.next_cp()?;
/* FP:format_foreign.rs-0375 */                 c = c_;
/* FP:format_foreign.rs-0376 */                 next = next_;
/* FP:format_foreign.rs-0377 */             }};
/* FP:format_foreign.rs-0378 */         }
/* FP:format_foreign.rs-0379 */ 
/* FP:format_foreign.rs-0380 */         // Constructs a result when parsing fails.
/* FP:format_foreign.rs-0381 */         //
/* FP:format_foreign.rs-0382 */         // Note: `move` used to capture copies of the cursors as they are *now*.
/* FP:format_foreign.rs-0383 */         let fallback = move || {
/* FP:format_foreign.rs-0384 */             Some((
/* FP:format_foreign.rs-0385 */                 Substitution::Format(Format {
/* FP:format_foreign.rs-0386 */                     span: start.slice_between(next).unwrap(),
/* FP:format_foreign.rs-0387 */                     parameter: None,
/* FP:format_foreign.rs-0388 */                     flags: "",
/* FP:format_foreign.rs-0389 */                     width: None,
/* FP:format_foreign.rs-0390 */                     precision: None,
/* FP:format_foreign.rs-0391 */                     length: None,
/* FP:format_foreign.rs-0392 */                     type_: at.slice_between(next).unwrap(),
/* FP:format_foreign.rs-0393 */                     position: InnerSpan::new(start.at, next.at),
/* FP:format_foreign.rs-0394 */                 }),
/* FP:format_foreign.rs-0395 */                 next.slice_after(),
/* FP:format_foreign.rs-0396 */             ))
/* FP:format_foreign.rs-0397 */         };
/* FP:format_foreign.rs-0398 */ 
/* FP:format_foreign.rs-0399 */         // Next parsing state.
/* FP:format_foreign.rs-0400 */         let mut state = Start;
/* FP:format_foreign.rs-0401 */ 
/* FP:format_foreign.rs-0402 */         // Sadly, Rust isn't *quite* smart enough to know these *must* be initialised by the end.
/* FP:format_foreign.rs-0403 */         let mut parameter: Option<u16> = None;
/* FP:format_foreign.rs-0404 */         let mut flags: &str = "";
/* FP:format_foreign.rs-0405 */         let mut width: Option<Num> = None;
/* FP:format_foreign.rs-0406 */         let mut precision: Option<Num> = None;
/* FP:format_foreign.rs-0407 */         let mut length: Option<&str> = None;
/* FP:format_foreign.rs-0408 */         let mut type_: &str = "";
/* FP:format_foreign.rs-0409 */         let end: Cur<'_>;
/* FP:format_foreign.rs-0410 */ 
/* FP:format_foreign.rs-0411 */         if let Start = state {
/* FP:format_foreign.rs-0412 */             match c {
/* FP:format_foreign.rs-0413 */                 '1'..='9' => {
/* FP:format_foreign.rs-0414 */                     let end = at_next_cp_while(next, char::is_ascii_digit);
/* FP:format_foreign.rs-0415 */                     match end.next_cp() {
/* FP:format_foreign.rs-0416 */                         // Yes, this *is* the parameter.
/* FP:format_foreign.rs-0417 */                         Some(('$', end2)) => {
/* FP:format_foreign.rs-0418 */                             state = Flags;
/* FP:format_foreign.rs-0419 */                             parameter = at.slice_between(end).unwrap().parse().ok();
/* FP:format_foreign.rs-0420 */                             move_to!(end2);
/* FP:format_foreign.rs-0421 */                         }
/* FP:format_foreign.rs-0422 */                         // Wait, no, actually, it's the width.
/* FP:format_foreign.rs-0423 */                         Some(_) => {
/* FP:format_foreign.rs-0424 */                             state = Prec;
/* FP:format_foreign.rs-0425 */                             parameter = None;
/* FP:format_foreign.rs-0426 */                             flags = "";
/* FP:format_foreign.rs-0427 */                             width = at.slice_between(end).and_then(|num| Num::from_str(num, None));
/* FP:format_foreign.rs-0428 */                             if width.is_none() {
/* FP:format_foreign.rs-0429 */                                 return fallback();
/* FP:format_foreign.rs-0430 */                             }
/* FP:format_foreign.rs-0431 */                             move_to!(end);
/* FP:format_foreign.rs-0432 */                         }
/* FP:format_foreign.rs-0433 */                         // It's invalid, is what it is.
/* FP:format_foreign.rs-0434 */                         None => return fallback(),
/* FP:format_foreign.rs-0435 */                     }
/* FP:format_foreign.rs-0436 */                 }
/* FP:format_foreign.rs-0437 */                 _ => {
/* FP:format_foreign.rs-0438 */                     state = Flags;
/* FP:format_foreign.rs-0439 */                     parameter = None;
/* FP:format_foreign.rs-0440 */                     move_to!(at);
/* FP:format_foreign.rs-0441 */                 }
/* FP:format_foreign.rs-0442 */             }
/* FP:format_foreign.rs-0443 */         }
/* FP:format_foreign.rs-0444 */ 
/* FP:format_foreign.rs-0445 */         if let Flags = state {
/* FP:format_foreign.rs-0446 */             let end = at_next_cp_while(at, is_flag);
/* FP:format_foreign.rs-0447 */             state = Width;
/* FP:format_foreign.rs-0448 */             flags = at.slice_between(end).unwrap();
/* FP:format_foreign.rs-0449 */             move_to!(end);
/* FP:format_foreign.rs-0450 */         }
/* FP:format_foreign.rs-0451 */ 
/* FP:format_foreign.rs-0452 */         if let Width = state {
/* FP:format_foreign.rs-0453 */             match c {
/* FP:format_foreign.rs-0454 */                 '*' => {
/* FP:format_foreign.rs-0455 */                     state = WidthArg;
/* FP:format_foreign.rs-0456 */                     move_to!(next);
/* FP:format_foreign.rs-0457 */                 }
/* FP:format_foreign.rs-0458 */                 '1'..='9' => {
/* FP:format_foreign.rs-0459 */                     let end = at_next_cp_while(next, char::is_ascii_digit);
/* FP:format_foreign.rs-0460 */                     state = Prec;
/* FP:format_foreign.rs-0461 */                     width = at.slice_between(end).and_then(|num| Num::from_str(num, None));
/* FP:format_foreign.rs-0462 */                     if width.is_none() {
/* FP:format_foreign.rs-0463 */                         return fallback();
/* FP:format_foreign.rs-0464 */                     }
/* FP:format_foreign.rs-0465 */                     move_to!(end);
/* FP:format_foreign.rs-0466 */                 }
/* FP:format_foreign.rs-0467 */                 _ => {
/* FP:format_foreign.rs-0468 */                     state = Prec;
/* FP:format_foreign.rs-0469 */                     width = None;
/* FP:format_foreign.rs-0470 */                     move_to!(at);
/* FP:format_foreign.rs-0471 */                 }
/* FP:format_foreign.rs-0472 */             }
/* FP:format_foreign.rs-0473 */         }
/* FP:format_foreign.rs-0474 */ 
/* FP:format_foreign.rs-0475 */         if let WidthArg = state {
/* FP:format_foreign.rs-0476 */             let end = at_next_cp_while(at, char::is_ascii_digit);
/* FP:format_foreign.rs-0477 */             match end.next_cp() {
/* FP:format_foreign.rs-0478 */                 Some(('$', end2)) => {
/* FP:format_foreign.rs-0479 */                     state = Prec;
/* FP:format_foreign.rs-0480 */                     width = Num::from_str("", at.slice_between(end));
/* FP:format_foreign.rs-0481 */                     move_to!(end2);
/* FP:format_foreign.rs-0482 */                 }
/* FP:format_foreign.rs-0483 */                 _ => {
/* FP:format_foreign.rs-0484 */                     state = Prec;
/* FP:format_foreign.rs-0485 */                     width = Some(Num::Next);
/* FP:format_foreign.rs-0486 */                     move_to!(end);
/* FP:format_foreign.rs-0487 */                 }
/* FP:format_foreign.rs-0488 */             }
/* FP:format_foreign.rs-0489 */         }
/* FP:format_foreign.rs-0490 */ 
/* FP:format_foreign.rs-0491 */         if let Prec = state {
/* FP:format_foreign.rs-0492 */             match c {
/* FP:format_foreign.rs-0493 */                 '.' => {
/* FP:format_foreign.rs-0494 */                     state = PrecInner;
/* FP:format_foreign.rs-0495 */                     move_to!(next);
/* FP:format_foreign.rs-0496 */                 }
/* FP:format_foreign.rs-0497 */                 _ => {
/* FP:format_foreign.rs-0498 */                     state = Length;
/* FP:format_foreign.rs-0499 */                     precision = None;
/* FP:format_foreign.rs-0500 */                     move_to!(at);
/* FP:format_foreign.rs-0501 */                 }
/* FP:format_foreign.rs-0502 */             }
/* FP:format_foreign.rs-0503 */         }
/* FP:format_foreign.rs-0504 */ 
/* FP:format_foreign.rs-0505 */         if let PrecInner = state {
/* FP:format_foreign.rs-0506 */             match c {
/* FP:format_foreign.rs-0507 */                 '*' => {
/* FP:format_foreign.rs-0508 */                     let end = at_next_cp_while(next, char::is_ascii_digit);
/* FP:format_foreign.rs-0509 */                     match end.next_cp() {
/* FP:format_foreign.rs-0510 */                         Some(('$', end2)) => {
/* FP:format_foreign.rs-0511 */                             state = Length;
/* FP:format_foreign.rs-0512 */                             precision = Num::from_str("*", next.slice_between(end));
/* FP:format_foreign.rs-0513 */                             move_to!(end2);
/* FP:format_foreign.rs-0514 */                         }
/* FP:format_foreign.rs-0515 */                         _ => {
/* FP:format_foreign.rs-0516 */                             state = Length;
/* FP:format_foreign.rs-0517 */                             precision = Some(Num::Next);
/* FP:format_foreign.rs-0518 */                             move_to!(end);
/* FP:format_foreign.rs-0519 */                         }
/* FP:format_foreign.rs-0520 */                     }
/* FP:format_foreign.rs-0521 */                 }
/* FP:format_foreign.rs-0522 */                 '0'..='9' => {
/* FP:format_foreign.rs-0523 */                     let end = at_next_cp_while(next, char::is_ascii_digit);
/* FP:format_foreign.rs-0524 */                     state = Length;
/* FP:format_foreign.rs-0525 */                     precision = at.slice_between(end).and_then(|num| Num::from_str(num, None));
/* FP:format_foreign.rs-0526 */                     move_to!(end);
/* FP:format_foreign.rs-0527 */                 }
/* FP:format_foreign.rs-0528 */                 _ => return fallback(),
/* FP:format_foreign.rs-0529 */             }
/* FP:format_foreign.rs-0530 */         }
/* FP:format_foreign.rs-0531 */ 
/* FP:format_foreign.rs-0532 */         if let Length = state {
/* FP:format_foreign.rs-0533 */             let c1_next1 = next.next_cp();
/* FP:format_foreign.rs-0534 */             match (c, c1_next1) {
/* FP:format_foreign.rs-0535 */                 ('h', Some(('h', next1))) | ('l', Some(('l', next1))) => {
/* FP:format_foreign.rs-0536 */                     state = Type;
/* FP:format_foreign.rs-0537 */                     length = Some(at.slice_between(next1).unwrap());
/* FP:format_foreign.rs-0538 */                     move_to!(next1);
/* FP:format_foreign.rs-0539 */                 }
/* FP:format_foreign.rs-0540 */ 
/* FP:format_foreign.rs-0541 */                 ('h' | 'l' | 'L' | 'z' | 'j' | 't' | 'q', _) => {
/* FP:format_foreign.rs-0542 */                     state = Type;
/* FP:format_foreign.rs-0543 */                     length = Some(at.slice_between(next).unwrap());
/* FP:format_foreign.rs-0544 */                     move_to!(next);
/* FP:format_foreign.rs-0545 */                 }
/* FP:format_foreign.rs-0546 */ 
/* FP:format_foreign.rs-0547 */                 ('I', _) => {
/* FP:format_foreign.rs-0548 */                     let end = next
/* FP:format_foreign.rs-0549 */                         .at_next_cp()
/* FP:format_foreign.rs-0550 */                         .and_then(|end| end.at_next_cp())
/* FP:format_foreign.rs-0551 */                         .map(|end| (next.slice_between(end).unwrap(), end));
/* FP:format_foreign.rs-0552 */                     let end = match end {
/* FP:format_foreign.rs-0553 */                         Some(("32" | "64", end)) => end,
/* FP:format_foreign.rs-0554 */                         _ => next,
/* FP:format_foreign.rs-0555 */                     };
/* FP:format_foreign.rs-0556 */                     state = Type;
/* FP:format_foreign.rs-0557 */                     length = Some(at.slice_between(end).unwrap());
/* FP:format_foreign.rs-0558 */                     move_to!(end);
/* FP:format_foreign.rs-0559 */                 }
/* FP:format_foreign.rs-0560 */ 
/* FP:format_foreign.rs-0561 */                 _ => {
/* FP:format_foreign.rs-0562 */                     state = Type;
/* FP:format_foreign.rs-0563 */                     length = None;
/* FP:format_foreign.rs-0564 */                     move_to!(at);
/* FP:format_foreign.rs-0565 */                 }
/* FP:format_foreign.rs-0566 */             }
/* FP:format_foreign.rs-0567 */         }
/* FP:format_foreign.rs-0568 */ 
/* FP:format_foreign.rs-0569 */         if let Type = state {
/* FP:format_foreign.rs-0570 */             type_ = at.slice_between(next).unwrap();
/* FP:format_foreign.rs-0571 */ 
/* FP:format_foreign.rs-0572 */             // Don't use `move_to!` here, as we *can* be at the end of the input.
/* FP:format_foreign.rs-0573 */             at = next;
/* FP:format_foreign.rs-0574 */         }
/* FP:format_foreign.rs-0575 */ 
/* FP:format_foreign.rs-0576 */         let _ = c; // to avoid never used value
/* FP:format_foreign.rs-0577 */ 
/* FP:format_foreign.rs-0578 */         end = at;
/* FP:format_foreign.rs-0579 */         let position = InnerSpan::new(start.at, end.at);
/* FP:format_foreign.rs-0580 */ 
/* FP:format_foreign.rs-0581 */         let f = Format {
/* FP:format_foreign.rs-0582 */             span: start.slice_between(end).unwrap(),
/* FP:format_foreign.rs-0583 */             parameter,
/* FP:format_foreign.rs-0584 */             flags,
/* FP:format_foreign.rs-0585 */             width,
/* FP:format_foreign.rs-0586 */             precision,
/* FP:format_foreign.rs-0587 */             length,
/* FP:format_foreign.rs-0588 */             type_,
/* FP:format_foreign.rs-0589 */             position,
/* FP:format_foreign.rs-0590 */         };
/* FP:format_foreign.rs-0591 */         Some((Substitution::Format(f), end.slice_after()))
/* FP:format_foreign.rs-0592 */     }
/* FP:format_foreign.rs-0593 */ 
/* FP:format_foreign.rs-0594 */     fn at_next_cp_while<F>(mut cur: Cur<'_>, mut pred: F) -> Cur<'_>
/* FP:format_foreign.rs-0595 */     where
/* FP:format_foreign.rs-0596 */         F: FnMut(&char) -> bool,
/* FP:format_foreign.rs-0597 */     {
/* FP:format_foreign.rs-0598 */         loop {
/* FP:format_foreign.rs-0599 */             match cur.next_cp() {
/* FP:format_foreign.rs-0600 */                 Some((c, next)) => {
/* FP:format_foreign.rs-0601 */                     if pred(&c) {
/* FP:format_foreign.rs-0602 */                         cur = next;
/* FP:format_foreign.rs-0603 */                     } else {
/* FP:format_foreign.rs-0604 */                         return cur;
/* FP:format_foreign.rs-0605 */                     }
/* FP:format_foreign.rs-0606 */                 }
/* FP:format_foreign.rs-0607 */                 None => return cur,
/* FP:format_foreign.rs-0608 */             }
/* FP:format_foreign.rs-0609 */         }
/* FP:format_foreign.rs-0610 */     }
/* FP:format_foreign.rs-0611 */ 
/* FP:format_foreign.rs-0612 */     fn is_flag(c: &char) -> bool {
/* FP:format_foreign.rs-0613 */         matches!(c, '0' | '-' | '+' | ' ' | '#' | '\'')
/* FP:format_foreign.rs-0614 */     }
/* FP:format_foreign.rs-0615 */ 
/* FP:format_foreign.rs-0616 */     #[cfg(test)]
/* FP:format_foreign.rs-0618 */ }
/* FP:format_foreign.rs-0619 */ 
/* FP:format_foreign.rs-0620 */ pub(crate) mod shell {
/* FP:format_foreign.rs-0621 */     use crate::rustc_complete::InnerSpan;
/* FP:format_foreign.rs-0622 */ 
/* FP:format_foreign.rs-0623 */     use super::strcursor::StrCursor as Cur;
/* FP:format_foreign.rs-0624 */ 
/* FP:format_foreign.rs-0625 */     #[derive(Clone, PartialEq, Debug)]
/* FP:format_foreign.rs-0626 */     pub(crate) enum Substitution<'a> {
/* FP:format_foreign.rs-0627 */         Ordinal(u8, (usize, usize)),
/* FP:format_foreign.rs-0628 */         Name(&'a str, (usize, usize)),
/* FP:format_foreign.rs-0629 */         Escape((usize, usize)),
/* FP:format_foreign.rs-0630 */     }
/* FP:format_foreign.rs-0631 */ 
/* FP:format_foreign.rs-0632 */     impl ToString for Substitution<'_> {
/* FP:format_foreign.rs-0633 */         fn to_string(&self) -> String {
/* FP:format_foreign.rs-0634 */             match self {
/* FP:format_foreign.rs-0635 */                 Substitution::Ordinal(n, _) => format!("${n}"),
/* FP:format_foreign.rs-0636 */                 Substitution::Name(n, _) => format!("${n}"),
/* FP:format_foreign.rs-0637 */                 Substitution::Escape(_) => "$$".into(),
/* FP:format_foreign.rs-0638 */             }
/* FP:format_foreign.rs-0639 */         }
/* FP:format_foreign.rs-0640 */     }
/* FP:format_foreign.rs-0641 */ 
/* FP:format_foreign.rs-0642 */     impl Substitution<'_> {
/* FP:format_foreign.rs-0643 */         pub(crate) fn position(&self) -> InnerSpan {
/* FP:format_foreign.rs-0644 */             let (Self::Ordinal(_, pos) | Self::Name(_, pos) | Self::Escape(pos)) = self;
/* FP:format_foreign.rs-0645 */             InnerSpan::new(pos.0, pos.1)
/* FP:format_foreign.rs-0646 */         }
/* FP:format_foreign.rs-0647 */ 
/* FP:format_foreign.rs-0648 */         fn set_position(&mut self, start: usize, end: usize) {
/* FP:format_foreign.rs-0649 */             let (Self::Ordinal(_, pos) | Self::Name(_, pos) | Self::Escape(pos)) = self;
/* FP:format_foreign.rs-0650 */             *pos = (start, end);
/* FP:format_foreign.rs-0651 */         }
/* FP:format_foreign.rs-0652 */ 
/* FP:format_foreign.rs-0653 */         pub(crate) fn translate(&self) -> Result<String, Option<String>> {
/* FP:format_foreign.rs-0654 */             match self {
/* FP:format_foreign.rs-0655 */                 Substitution::Ordinal(n, _) => Ok(format!("{{{}}}", n)),
/* FP:format_foreign.rs-0656 */                 Substitution::Name(n, _) => Ok(format!("{{{}}}", n)),
/* FP:format_foreign.rs-0657 */                 Substitution::Escape(_) => Err(None),
/* FP:format_foreign.rs-0658 */             }
/* FP:format_foreign.rs-0659 */         }
/* FP:format_foreign.rs-0660 */     }
/* FP:format_foreign.rs-0661 */ 
/* FP:format_foreign.rs-0662 */     /// Returns an iterator over all substitutions in a given string.
/* FP:format_foreign.rs-0663 */     pub(crate) fn iter_subs(s: &str, start_pos: usize) -> Substitutions<'_> {
/* FP:format_foreign.rs-0664 */         Substitutions { s, pos: start_pos }
/* FP:format_foreign.rs-0665 */     }
/* FP:format_foreign.rs-0666 */ 
/* FP:format_foreign.rs-0667 */     /// Iterator over substitutions in a string.
/* FP:format_foreign.rs-0668 */     pub(crate) struct Substitutions<'a> {
/* FP:format_foreign.rs-0669 */         s: &'a str,
/* FP:format_foreign.rs-0670 */         pos: usize,
/* FP:format_foreign.rs-0671 */     }
/* FP:format_foreign.rs-0672 */ 
/* FP:format_foreign.rs-0673 */     impl<'a> Iterator for Substitutions<'a> {
/* FP:format_foreign.rs-0674 */         type Item = Substitution<'a>;
/* FP:format_foreign.rs-0675 */         fn next(&mut self) -> Option<Self::Item> {
/* FP:format_foreign.rs-0676 */             let (mut sub, tail) = parse_next_substitution(self.s)?;
/* FP:format_foreign.rs-0677 */             self.s = tail;
/* FP:format_foreign.rs-0678 */             let InnerSpan { start, end } = sub.position();
/* FP:format_foreign.rs-0679 */             sub.set_position(start + self.pos, end + self.pos);
/* FP:format_foreign.rs-0680 */             self.pos += end;
/* FP:format_foreign.rs-0681 */             Some(sub)
/* FP:format_foreign.rs-0682 */         }
/* FP:format_foreign.rs-0683 */ 
/* FP:format_foreign.rs-0684 */         fn size_hint(&self) -> (usize, Option<usize>) {
/* FP:format_foreign.rs-0685 */             (0, Some(self.s.len()))
/* FP:format_foreign.rs-0686 */         }
/* FP:format_foreign.rs-0687 */     }
/* FP:format_foreign.rs-0688 */ 
/* FP:format_foreign.rs-0689 */     /// Parse the next substitution from the input string.
/* FP:format_foreign.rs-0690 */     fn parse_next_substitution(s: &str) -> Option<(Substitution<'_>, &str)> {
/* FP:format_foreign.rs-0691 */         let at = {
/* FP:format_foreign.rs-0692 */             let start = s.find('$')?;
/* FP:format_foreign.rs-0693 */             match s[start + 1..].chars().next()? {
/* FP:format_foreign.rs-0694 */                 '$' => return Some((Substitution::Escape((start, start + 2)), &s[start + 2..])),
/* FP:format_foreign.rs-0695 */                 c @ '0'..='9' => {
/* FP:format_foreign.rs-0696 */                     let n = (c as u8) - b'0';
/* FP:format_foreign.rs-0697 */                     return Some((Substitution::Ordinal(n, (start, start + 2)), &s[start + 2..]));
/* FP:format_foreign.rs-0698 */                 }
/* FP:format_foreign.rs-0699 */                 _ => { /* fall-through */ }
/* FP:format_foreign.rs-0700 */             }
/* FP:format_foreign.rs-0701 */ 
/* FP:format_foreign.rs-0702 */             Cur::new_at(s, start)
/* FP:format_foreign.rs-0703 */         };
/* FP:format_foreign.rs-0704 */ 
/* FP:format_foreign.rs-0705 */         let at = at.at_next_cp()?;
/* FP:format_foreign.rs-0706 */         let (c, inner) = at.next_cp()?;
/* FP:format_foreign.rs-0707 */ 
/* FP:format_foreign.rs-0708 */         if !is_ident_head(c) {
/* FP:format_foreign.rs-0709 */             None
/* FP:format_foreign.rs-0710 */         } else {
/* FP:format_foreign.rs-0711 */             let end = at_next_cp_while(inner, is_ident_tail);
/* FP:format_foreign.rs-0712 */             let slice = at.slice_between(end).unwrap();
/* FP:format_foreign.rs-0713 */             let start = at.at - 1;
/* FP:format_foreign.rs-0714 */             let end_pos = at.at + slice.len();
/* FP:format_foreign.rs-0715 */             Some((Substitution::Name(slice, (start, end_pos)), end.slice_after()))
/* FP:format_foreign.rs-0716 */         }
/* FP:format_foreign.rs-0717 */     }
/* FP:format_foreign.rs-0718 */ 
/* FP:format_foreign.rs-0719 */     fn at_next_cp_while<F>(mut cur: Cur<'_>, mut pred: F) -> Cur<'_>
/* FP:format_foreign.rs-0720 */     where
/* FP:format_foreign.rs-0721 */         F: FnMut(char) -> bool,
/* FP:format_foreign.rs-0722 */     {
/* FP:format_foreign.rs-0723 */         loop {
/* FP:format_foreign.rs-0724 */             match cur.next_cp() {
/* FP:format_foreign.rs-0725 */                 Some((c, next)) => {
/* FP:format_foreign.rs-0726 */                     if pred(c) {
/* FP:format_foreign.rs-0727 */                         cur = next;
/* FP:format_foreign.rs-0728 */                     } else {
/* FP:format_foreign.rs-0729 */                         return cur;
/* FP:format_foreign.rs-0730 */                     }
/* FP:format_foreign.rs-0731 */                 }
/* FP:format_foreign.rs-0732 */                 None => return cur,
/* FP:format_foreign.rs-0733 */             }
/* FP:format_foreign.rs-0734 */         }
/* FP:format_foreign.rs-0735 */     }
/* FP:format_foreign.rs-0736 */ 
/* FP:format_foreign.rs-0737 */     fn is_ident_head(c: char) -> bool {
/* FP:format_foreign.rs-0738 */         c.is_ascii_alphabetic() || c == '_'
/* FP:format_foreign.rs-0739 */     }
/* FP:format_foreign.rs-0740 */ 
/* FP:format_foreign.rs-0741 */     fn is_ident_tail(c: char) -> bool {
/* FP:format_foreign.rs-0742 */         c.is_ascii_alphanumeric() || c == '_'
/* FP:format_foreign.rs-0743 */     }
/* FP:format_foreign.rs-0744 */ 
/* FP:format_foreign.rs-0745 */     #[cfg(test)]
/* FP:format_foreign.rs-0747 */ }
/* FP:format_foreign.rs-0748 */ 
/* FP:format_foreign.rs-0749 */ mod strcursor {
/* FP:format_foreign.rs-0750 */     pub(crate) struct StrCursor<'a> {
/* FP:format_foreign.rs-0751 */         s: &'a str,
/* FP:format_foreign.rs-0752 */         pub at: usize,
/* FP:format_foreign.rs-0753 */     }
/* FP:format_foreign.rs-0754 */ 
/* FP:format_foreign.rs-0755 */     impl<'a> StrCursor<'a> {
/* FP:format_foreign.rs-0756 */         pub(crate) fn new_at(s: &'a str, at: usize) -> StrCursor<'a> {
/* FP:format_foreign.rs-0757 */             StrCursor { s, at }
/* FP:format_foreign.rs-0758 */         }
/* FP:format_foreign.rs-0759 */ 
/* FP:format_foreign.rs-0760 */         pub(crate) fn at_next_cp(mut self) -> Option<StrCursor<'a>> {
/* FP:format_foreign.rs-0761 */             match self.try_seek_right_cp() {
/* FP:format_foreign.rs-0762 */                 true => Some(self),
/* FP:format_foreign.rs-0763 */                 false => None,
/* FP:format_foreign.rs-0764 */             }
/* FP:format_foreign.rs-0765 */         }
/* FP:format_foreign.rs-0766 */ 
/* FP:format_foreign.rs-0767 */         pub(crate) fn next_cp(mut self) -> Option<(char, StrCursor<'a>)> {
/* FP:format_foreign.rs-0768 */             let cp = self.cp_after()?;
/* FP:format_foreign.rs-0769 */             self.seek_right(cp.len_utf8());
/* FP:format_foreign.rs-0770 */             Some((cp, self))
/* FP:format_foreign.rs-0771 */         }
/* FP:format_foreign.rs-0772 */ 
/* FP:format_foreign.rs-0773 */         fn slice_before(&self) -> &'a str {
/* FP:format_foreign.rs-0774 */             &self.s[0..self.at]
/* FP:format_foreign.rs-0775 */         }
/* FP:format_foreign.rs-0776 */ 
/* FP:format_foreign.rs-0777 */         pub(crate) fn slice_after(&self) -> &'a str {
/* FP:format_foreign.rs-0778 */             &self.s[self.at..]
/* FP:format_foreign.rs-0779 */         }
/* FP:format_foreign.rs-0780 */ 
/* FP:format_foreign.rs-0781 */         pub(crate) fn slice_between(&self, until: StrCursor<'a>) -> Option<&'a str> {
/* FP:format_foreign.rs-0782 */             if !str_eq_literal(self.s, until.s) {
/* FP:format_foreign.rs-0783 */                 None
/* FP:format_foreign.rs-0784 */             } else {
/* FP:format_foreign.rs-0785 */                 use std::cmp::{max, min};
/* FP:format_foreign.rs-0786 */                 let beg = min(self.at, until.at);
/* FP:format_foreign.rs-0787 */                 let end = max(self.at, until.at);
/* FP:format_foreign.rs-0788 */                 Some(&self.s[beg..end])
/* FP:format_foreign.rs-0789 */             }
/* FP:format_foreign.rs-0790 */         }
/* FP:format_foreign.rs-0791 */ 
/* FP:format_foreign.rs-0792 */         fn cp_after(&self) -> Option<char> {
/* FP:format_foreign.rs-0793 */             self.slice_after().chars().next()
/* FP:format_foreign.rs-0794 */         }
/* FP:format_foreign.rs-0795 */ 
/* FP:format_foreign.rs-0796 */         fn try_seek_right_cp(&mut self) -> bool {
/* FP:format_foreign.rs-0797 */             match self.slice_after().chars().next() {
/* FP:format_foreign.rs-0798 */                 Some(c) => {
/* FP:format_foreign.rs-0799 */                     self.at += c.len_utf8();
/* FP:format_foreign.rs-0800 */                     true
/* FP:format_foreign.rs-0801 */                 }
/* FP:format_foreign.rs-0802 */                 None => false,
/* FP:format_foreign.rs-0803 */             }
/* FP:format_foreign.rs-0804 */         }
/* FP:format_foreign.rs-0805 */ 
/* FP:format_foreign.rs-0806 */         fn seek_right(&mut self, bytes: usize) {
/* FP:format_foreign.rs-0807 */             self.at += bytes;
/* FP:format_foreign.rs-0808 */         }
/* FP:format_foreign.rs-0809 */     }
/* FP:format_foreign.rs-0810 */ 
/* FP:format_foreign.rs-0811 */     impl Copy for StrCursor<'_> {}
/* FP:format_foreign.rs-0812 */ 
/* FP:format_foreign.rs-0813 */     impl<'a> Clone for StrCursor<'a> {
/* FP:format_foreign.rs-0814 */         fn clone(&self) -> StrCursor<'a> {
/* FP:format_foreign.rs-0815 */             *self
/* FP:format_foreign.rs-0816 */         }
/* FP:format_foreign.rs-0817 */     }
/* FP:format_foreign.rs-0818 */ 
/* FP:format_foreign.rs-0819 */     impl std::fmt::Debug for StrCursor<'_> {
/* FP:format_foreign.rs-0820 */         fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
/* FP:format_foreign.rs-0821 */             write!(fmt, "StrCursor({:?} | {:?})", self.slice_before(), self.slice_after())
/* FP:format_foreign.rs-0822 */         }
/* FP:format_foreign.rs-0823 */     }
/* FP:format_foreign.rs-0824 */ 
/* FP:format_foreign.rs-0825 */     fn str_eq_literal(a: &str, b: &str) -> bool {
/* FP:format_foreign.rs-0826 */         a.as_bytes().as_ptr() == b.as_bytes().as_ptr() && a.len() == b.len()
/* FP:format_foreign.rs-0827 */     }
/* FP:format_foreign.rs-0828 */ }