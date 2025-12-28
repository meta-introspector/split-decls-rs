macro_rules! deps {
    () => {
        Error!();
        Result!();
        Out!();
        MapAccess!();
        DeserializeSeed!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < 'de > serde :: de :: MapAccess < 'de > for & mut (dyn MapAccess < 'de > + '_) { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Error > where K : serde :: de :: DeserializeSeed < 'de > , { let mut erased = erase :: DeserializeSeed :: new (seed) ; unsafe { (* * self) . erased_next_key (& mut erased) . map (| opt | opt . unsafe_map (Out :: take)) } } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Error > where V : serde :: de :: DeserializeSeed < 'de > , { let mut erased = erase :: DeserializeSeed :: new (seed) ; unsafe { (* * self) . erased_next_value (& mut erased) . unsafe_map (Out :: take) } } fn size_hint (& self) -> Option < usize > { (* * self) . erased_size_hint () } }
    };
}

impl_44!();