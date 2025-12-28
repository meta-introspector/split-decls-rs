macro_rules! RE_SPLIT_IDENT {
    () => {
        static RE_SPLIT_IDENT : Lazy < Regex > = Lazy :: new (| | { Regex :: new (r"[^a-zA-Z0-9_]+") . unwrap () }) ;
    };
}

RE_SPLIT_IDENT!();