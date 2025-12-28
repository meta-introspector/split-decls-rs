macro_rules! impl_187 {
    () => {
        impl ProvidedIdx { pub (crate) fn to_expected_idx (self) -> ExpectedIdx { ExpectedIdx :: from_u32 (self . as_u32 ()) } }
    };
}

impl_187!();