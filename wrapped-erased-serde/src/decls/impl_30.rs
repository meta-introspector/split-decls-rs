macro_rules! deps {
    () => {
        Result!();
        Error!();
        DeserializeSeed!();
        Out!();
        MapAccess!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < 'de , T > MapAccess < 'de > for erase :: MapAccess < T > where T : serde :: de :: MapAccess < 'de > , { fn erased_next_key (& mut self , seed : & mut dyn DeserializeSeed < 'de > ,) -> Result < Option < Out > , Error > { self . as_mut () . next_key_seed (seed) . map_err (erase) } fn erased_next_value (& mut self , seed : & mut dyn DeserializeSeed < 'de >) -> Result < Out , Error > { self . as_mut () . next_value_seed (seed) . map_err (erase) } fn erased_next_entry (& mut self , kseed : & mut dyn DeserializeSeed < 'de > , vseed : & mut dyn DeserializeSeed < 'de > ,) -> Result < Option < (Out , Out) > , Error > { self . as_mut () . next_entry_seed (kseed , vseed) . map_err (erase) } fn erased_size_hint (& self) -> Option < usize > { self . as_ref () . size_hint () } }
    };
}

impl_30!()