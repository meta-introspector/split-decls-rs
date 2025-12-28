macro_rules! deps {
    () => {
        PropertyEnumToValueNameLookup!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl PropertyEnumToValueNameLookup for PropertyEnumToValueNameLinearMap < '_ > { fn get (& self , prop : u32) -> Option < & str > { self . map . get (usize :: try_from (prop) . ok () ?) } }
    };
}

impl_256!();