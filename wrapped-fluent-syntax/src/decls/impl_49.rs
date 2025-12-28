macro_rules! deps {
    () => {
        Slice!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < 's > Slice < 's > for & 's str { fn slice (& self , range : Range < usize >) -> Self { & self [range] } fn trim (& mut self) { * self = self . trim_end_matches (matches_fluent_ws) ; } }
    };
}

impl_49!()