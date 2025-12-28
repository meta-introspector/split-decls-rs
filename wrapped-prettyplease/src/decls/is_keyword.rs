macro_rules! is_keyword {
    () => {
        fn is_keyword (ident : & Ident) -> bool { match ident . to_string () . as_str () { "as" | "async" | "await" | "box" | "break" | "const" | "continue" | "crate" | "dyn" | "else" | "enum" | "extern" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "static" | "struct" | "trait" | "type" | "unsafe" | "use" | "where" | "while" | "yield" => true , _ => false , } }
    };
}

is_keyword!()