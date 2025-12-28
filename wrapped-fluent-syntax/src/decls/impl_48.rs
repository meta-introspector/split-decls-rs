macro_rules! deps {
    () => {
        Slice!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl Slice < '_ > for String { fn slice (& self , range : Range < usize >) -> Self { self [range] . to_string () } fn trim (& mut self) { * self = self . trim_end_matches (matches_fluent_ws) . to_string () ; } }
    };
}

impl_48!()