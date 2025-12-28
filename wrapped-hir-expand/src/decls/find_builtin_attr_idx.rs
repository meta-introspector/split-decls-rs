macro_rules! find_builtin_attr_idx {
    () => {
        pub fn find_builtin_attr_idx (name : & Symbol) -> Option < usize > { static BUILTIN_LOOKUP_TABLE : OnceLock < FxHashMap < Symbol , usize > > = OnceLock :: new () ; BUILTIN_LOOKUP_TABLE . get_or_init (| | { INERT_ATTRIBUTES . iter () . map (| attr | attr . name) . enumerate () . map (| (a , b) | (Symbol :: intern (b) , a)) . collect () }) . get (name) . copied () }
    };
}

find_builtin_attr_idx!();