macro_rules! deps {
    () => {
        PropertyEnumToValueNameLookup!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl PropertyEnumToValueNameLookup for PropertyEnumToValueNameSparseMap < '_ > { fn get (& self , prop : u32) -> Option < & str > { self . map . get (& u16 :: try_from (prop) . ok () ?) } }
    };
}

impl_58!();