use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Expect {
    /// Checks if this expect is equal to `actual`.
    pub fn assert_eq(&self, actual: &str) {
        let trimmed = self.trimmed();
        if trimmed == actual {
            return;
        }
        Runtime::fail_expect(self, &trimmed, actual);
    }
    /// Checks if this expect is equal to `format!("{:#?}", actual)`.
    pub fn assert_debug_eq(&self, actual: &impl fmt::Debug) {
        let actual = format!("{:#?}\n", actual);
        self.assert_eq(&actual)
    }
    /// If `true` (default), in-place update will indent the string literal.
    pub fn indent(&mut self, yes: bool) {
        self.indent = yes;
    }
    /// Returns the content of this expect.
    pub fn data(&self) -> &str {
        self.data
    }
    fn trimmed(&self) -> String {
        if !self.data.contains('\n') {
            return self.data.to_string();
        }
        trim_indent(self.data)
    }
    fn locate(&self, file: &str) -> Location {
        let mut target_line = None;
        let mut line_start = 0;
        for (i, line) in lines_with_ends(file).enumerate() {
            if i == self.position.line as usize - 1 {
                let byte_offset = line
                    .char_indices()
                    .skip((self.position.column - 1).try_into().unwrap())
                    .skip_while(|&(_, c)| c != '!')
                    .skip(1)
                    .skip_while(|&(_, c)| c.is_whitespace())
                    .skip(1)
                    .skip_while(|&(_, c)| c.is_whitespace())
                    .next()
                    .expect("Failed to parse macro invocation")
                    .0;
                let literal_start = line_start + byte_offset;
                let indent = line.chars().take_while(|&it| it == ' ').count();
                target_line = Some((literal_start, indent));
                break;
            }
            line_start += line.len();
        }
        let (literal_start, line_indent) = target_line.unwrap();
        let lit_to_eof = &file[literal_start..];
        let lit_to_eof_trimmed = lit_to_eof.trim_start();
        let literal_start = literal_start
            + (lit_to_eof.len() - lit_to_eof_trimmed.len());
        let literal_len = locate_end(lit_to_eof_trimmed)
            .expect("Couldn't find closing delimiter for `expect!`.");
        let literal_range = literal_start..literal_start + literal_len;
        Location {
            line_indent,
            literal_range,
        }
    }
}
