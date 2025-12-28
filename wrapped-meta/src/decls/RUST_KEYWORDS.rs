macro_rules! RUST_KEYWORDS {
    () => {
        static RUST_KEYWORDS : LazyLock < HashSet < & 'static str > > = LazyLock :: new (| | { ["abstract" , "alignof" , "as" , "become" , "box" , "break" , "const" , "continue" , "crate" , "do" , "else" , "enum" , "extern" , "false" , "final" , "fn" , "for" , "if" , "impl" , "in" , "let" , "loop" , "macro" , "match" , "mod" , "move" , "mut" , "offsetof" , "override" , "priv" , "proc" , "pure" , "pub" , "ref" , "return" , "Self" , "self" , "sizeof" , "static" , "struct" , "super" , "trait" , "true" , "type" , "typeof" , "unsafe" , "unsized" , "use" , "virtual" , "where" , "while" , "yield" ,] . iter () . cloned () . collect () }) ;
    };
}

RUST_KEYWORDS!();