use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns the textual content of a doc comment block as a quoted string
/// That is, strips leading `///` (or `/**`, etc)
/// and strips the ending `*/`
/// And then quote the string, which is needed to convert to `tt::Literal`
///
/// Note that proc-macros desugar with string literals where as macro_rules macros desugar with raw string literals.
pub fn desugar_doc_comment_text(text: &str, mode: DocCommentDesugarMode) -> (Symbol, tt::LitKind) {
    match mode {
        DocCommentDesugarMode::Mbe => {
            let mut num_of_hashes = 0;
            let mut count = 0;
            for ch in text.chars() {
                count = match ch {
                    '"' => 1,
                    '#' if count > 0 => count + 1,
                    _ => 0,
                };
                num_of_hashes = num_of_hashes.max(count);
            }
            (Symbol::intern(text), tt::LitKind::StrRaw(num_of_hashes))
        }
        DocCommentDesugarMode::ProcMacro => (
            Symbol::intern(&format_smolstr!("{}", text.escape_debug())),
            tt::LitKind::Str,
        ),
    }
}
