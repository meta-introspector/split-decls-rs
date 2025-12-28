macro_rules! deps {
    () => {
        SeqAccess!();
        Error!();
        DeserializeSeed!();
        Out!();
        Result!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'de , T > SeqAccess < 'de > for erase :: SeqAccess < T > where T : serde :: de :: SeqAccess < 'de > , { fn erased_next_element (& mut self , seed : & mut dyn DeserializeSeed < 'de > ,) -> Result < Option < Out > , Error > { self . as_mut () . next_element_seed (seed) . map_err (erase) } fn erased_size_hint (& self) -> Option < usize > { self . as_ref () . size_hint () } }
    };
}

impl_29!()