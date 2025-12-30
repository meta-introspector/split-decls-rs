// Generated macro for is_keyword (function)
macro_rules! Depcrate_macis_keyword {
() => {
// Module: crate::mac
// Provides: {"is_keyword"}
// Dependencies: {}
fn is_keyword (ident : & Ident) -> bool { match ident . to_string () . as_str () { "as" | "async" | "await" | "box" | "break" | "const" | "continue" | "crate" | "dyn" | "else" | "enum" | "extern" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "static" | "struct" | "trait" | "type" | "unsafe" | "use" | "where" | "while" | "yield" => true , _ => false , } }
};
}
