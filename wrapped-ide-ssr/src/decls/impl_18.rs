macro_rules! impl_18 {
    () => {
        impl Match { pub fn matched_text (& self) -> String { self . matched_node . text () . to_string () } }
    };
}

impl_18!()