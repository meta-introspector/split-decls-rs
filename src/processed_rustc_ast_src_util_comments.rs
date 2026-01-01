/* FP:comments.rs-0001 */ use crate::rustc_complete::{BytePos, Symbol};
/* FP:comments.rs-0002 */ 
/* FP:comments.rs-0003 */ use crate::token::CommentKind;
/* FP:comments.rs-0004 */ 
/* FP:comments.rs-0005 */ #[cfg(test)]
/* FP:comments.rs-0007 */ 
/* FP:comments.rs-0008 */ #[derive(Clone, Copy, PartialEq, Debug)]
/* FP:comments.rs-0009 */ pub enum CommentStyle {
/* FP:comments.rs-0010 */     /// No code on either side of each line of the comment
/* FP:comments.rs-0011 */     Isolated,
/* FP:comments.rs-0012 */     /// Code exists to the left of the comment
/* FP:comments.rs-0013 */     Trailing,
/* FP:comments.rs-0014 */     /// Code before /* foo */ and after the comment
/* FP:comments.rs-0015 */     Mixed,
/* FP:comments.rs-0016 */     /// Just a manual blank line "\n\n", for layout
/* FP:comments.rs-0017 */     BlankLine,
/* FP:comments.rs-0018 */ }
/* FP:comments.rs-0019 */ 
/* FP:comments.rs-0020 */ #[derive(Clone)]
/* FP:comments.rs-0021 */ pub struct Comment {
/* FP:comments.rs-0022 */     pub style: CommentStyle,
/* FP:comments.rs-0023 */     pub lines: Vec<String>,
/* FP:comments.rs-0024 */     pub pos: BytePos,
/* FP:comments.rs-0025 */ }
/* FP:comments.rs-0026 */ 
/* FP:comments.rs-0027 */ /// A fast conservative estimate on whether the string can contain documentation links.
/* FP:comments.rs-0028 */ /// A pair of square brackets `[]` must exist in the string, but we only search for the
/* FP:comments.rs-0029 */ /// opening bracket because brackets always go in pairs in practice.
/* FP:comments.rs-0030 */ #[inline]
/* FP:comments.rs-0031 */ pub fn may_have_doc_links(s: &str) -> bool {
/* FP:comments.rs-0032 */     s.contains('[')
/* FP:comments.rs-0033 */ }
/* FP:comments.rs-0034 */ 
/* FP:comments.rs-0035 */ /// Makes a doc string more presentable to users.
/* FP:comments.rs-0036 */ /// Used by rustdoc and perhaps other tools, but not by rustc.
/* FP:comments.rs-0037 */ pub fn beautify_doc_string(data: Symbol, kind: CommentKind) -> Symbol {
/* FP:comments.rs-0038 */     fn get_vertical_trim(lines: &[&str]) -> Option<(usize, usize)> {
/* FP:comments.rs-0039 */         let mut i = 0;
/* FP:comments.rs-0040 */         let mut j = lines.len();
/* FP:comments.rs-0041 */         // first line of all-stars should be omitted
/* FP:comments.rs-0042 */         if lines.first().is_some_and(|line| line.chars().all(|c| c == '*')) {
/* FP:comments.rs-0043 */             i += 1;
/* FP:comments.rs-0044 */         }
/* FP:comments.rs-0045 */ 
/* FP:comments.rs-0046 */         // like the first, a last line of all stars should be omitted
/* FP:comments.rs-0047 */         if j > i && !lines[j - 1].is_empty() && lines[j - 1].chars().all(|c| c == '*') {
/* FP:comments.rs-0048 */             j -= 1;
/* FP:comments.rs-0049 */         }
/* FP:comments.rs-0050 */ 
/* FP:comments.rs-0051 */         if i != 0 || j != lines.len() { Some((i, j)) } else { None }
/* FP:comments.rs-0052 */     }
/* FP:comments.rs-0053 */ 
/* FP:comments.rs-0054 */     fn get_horizontal_trim(lines: &[&str], kind: CommentKind) -> Option<String> {
/* FP:comments.rs-0055 */         let mut i = usize::MAX;
/* FP:comments.rs-0056 */         let mut first = true;
/* FP:comments.rs-0057 */ 
/* FP:comments.rs-0058 */         // In case we have doc comments like `/**` or `/*`, we want to remove stars if they are
/* FP:comments.rs-0059 */         // present. However, we first need to strip the empty lines so they don't get in the middle
/* FP:comments.rs-0060 */         // when we try to compute the "horizontal trim".
/* FP:comments.rs-0061 */         let lines = match kind {
/* FP:comments.rs-0062 */             CommentKind::Block => {
/* FP:comments.rs-0063 */                 // Whatever happens, we skip the first line.
/* FP:comments.rs-0064 */                 let mut i = lines
/* FP:comments.rs-0065 */                     .first()
/* FP:comments.rs-0066 */                     .map(|l| if l.trim_start().starts_with('*') { 0 } else { 1 })
/* FP:comments.rs-0067 */                     .unwrap_or(0);
/* FP:comments.rs-0068 */                 let mut j = lines.len();
/* FP:comments.rs-0069 */ 
/* FP:comments.rs-0070 */                 while i < j && lines[i].trim().is_empty() {
/* FP:comments.rs-0071 */                     i += 1;
/* FP:comments.rs-0072 */                 }
/* FP:comments.rs-0073 */                 while j > i && lines[j - 1].trim().is_empty() {
/* FP:comments.rs-0074 */                     j -= 1;
/* FP:comments.rs-0075 */                 }
/* FP:comments.rs-0076 */                 &lines[i..j]
/* FP:comments.rs-0077 */             }
/* FP:comments.rs-0078 */             CommentKind::Line => lines,
/* FP:comments.rs-0079 */         };
/* FP:comments.rs-0080 */ 
/* FP:comments.rs-0081 */         for line in lines {
/* FP:comments.rs-0082 */             for (j, c) in line.chars().enumerate() {
/* FP:comments.rs-0083 */                 if j > i || !"* \t".contains(c) {
/* FP:comments.rs-0084 */                     return None;
/* FP:comments.rs-0085 */                 }
/* FP:comments.rs-0086 */                 if c == '*' {
/* FP:comments.rs-0087 */                     if first {
/* FP:comments.rs-0088 */                         i = j;
/* FP:comments.rs-0089 */                         first = false;
/* FP:comments.rs-0090 */                     } else if i != j {
/* FP:comments.rs-0091 */                         return None;
/* FP:comments.rs-0092 */                     }
/* FP:comments.rs-0093 */                     break;
/* FP:comments.rs-0094 */                 }
/* FP:comments.rs-0095 */             }
/* FP:comments.rs-0096 */             if i >= line.len() {
/* FP:comments.rs-0097 */                 return None;
/* FP:comments.rs-0098 */             }
/* FP:comments.rs-0099 */         }
/* FP:comments.rs-0100 */         Some(lines.first()?[..i].to_string())
/* FP:comments.rs-0101 */     }
/* FP:comments.rs-0102 */ 
/* FP:comments.rs-0103 */     let data_s = data.as_str();
/* FP:comments.rs-0104 */     if data_s.contains('\n') {
/* FP:comments.rs-0105 */         let mut lines = data_s.lines().collect::<Vec<&str>>();
/* FP:comments.rs-0106 */         let mut changes = false;
/* FP:comments.rs-0107 */         let lines = if let Some((i, j)) = get_vertical_trim(&lines) {
/* FP:comments.rs-0108 */             changes = true;
/* FP:comments.rs-0109 */             // remove whitespace-only lines from the start/end of lines
/* FP:comments.rs-0110 */             &mut lines[i..j]
/* FP:comments.rs-0111 */         } else {
/* FP:comments.rs-0112 */             &mut lines
/* FP:comments.rs-0113 */         };
/* FP:comments.rs-0114 */         if let Some(horizontal) = get_horizontal_trim(lines, kind) {
/* FP:comments.rs-0115 */             changes = true;
/* FP:comments.rs-0116 */             // remove a "[ \t]*\*" block from each line, if possible
/* FP:comments.rs-0117 */             for line in lines.iter_mut() {
/* FP:comments.rs-0118 */                 if let Some(tmp) = line.strip_prefix(&horizontal) {
/* FP:comments.rs-0119 */                     *line = tmp;
/* FP:comments.rs-0120 */                     if kind == CommentKind::Block
/* FP:comments.rs-0121 */                         && (*line == "*" || line.starts_with("* ") || line.starts_with("**"))
/* FP:comments.rs-0122 */                     {
/* FP:comments.rs-0123 */                         *line = &line[1..];
/* FP:comments.rs-0124 */                     }
/* FP:comments.rs-0125 */                 }
/* FP:comments.rs-0126 */             }
/* FP:comments.rs-0127 */         }
/* FP:comments.rs-0128 */         if changes {
/* FP:comments.rs-0129 */             return Symbol::intern(&lines.join("\n"));
/* FP:comments.rs-0130 */         }
/* FP:comments.rs-0131 */     }
/* FP:comments.rs-0132 */     data
/* FP:comments.rs-0133 */ }