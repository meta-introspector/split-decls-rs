macro_rules! deps {
    () => {
        Match!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl Match { pub fn matched_text (& self) -> String { self . matched_node . text () . to_string () } }
    };
}

impl_192!();