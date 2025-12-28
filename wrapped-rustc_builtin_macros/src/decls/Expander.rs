macro_rules! Expander {
    () => {
        pub (crate) struct Expander { pub is_const : bool , }
    };
}

Expander!();