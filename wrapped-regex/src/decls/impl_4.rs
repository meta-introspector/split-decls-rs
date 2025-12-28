macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Default for Builder { fn default () -> Builder { let metac = meta :: Config :: new () . nfa_size_limit (Some (10 * (1 << 20))) . hybrid_cache_capacity (2 * (1 << 20)) ; Builder { pats : vec ! [] , metac , syntaxc : syntax :: Config :: default () } } }
    };
}

impl_4!();