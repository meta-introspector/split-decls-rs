macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! to_ident {
    () => {
        deps!();
        pub fn to_ident (name : & str) -> TokenStream { match name { "abstract" | "as" | "become" | "box" | "break" | "const" | "continue" | "crate" | "do" | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "override" | "priv" | "pub" | "ref" | "return" | "static" | "struct" | "super" | "trait" | "true" | "type" | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield" | "try" | "async" | "await" | "dyn" => format ! ("r#{name}") . into () , "Self" | "self" => format ! ("{name}_") . into () , "_" => "unused" . into () , _ => name . into () , } }
    };
}

to_ident!();