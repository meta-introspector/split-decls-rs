macro_rules! impl_186 {
    () => {
        impl ExpectedIdx { pub (crate) fn to_provided_idx (self) -> ProvidedIdx { ProvidedIdx :: from_usize (self . as_usize ()) } }
    };
}

impl_186!()