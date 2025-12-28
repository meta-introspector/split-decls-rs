macro_rules! deps {
    () => {
        Out!();
        Result!();
        DeserializeSeed!();
        Error!();
    };
}

macro_rules! SeqAccess {
    () => {
        deps!();
        pub trait SeqAccess < 'de > { fn erased_next_element (& mut self , seed : & mut dyn DeserializeSeed < 'de > ,) -> Result < Option < Out > , Error > ; fn erased_size_hint (& self) -> Option < usize > ; }
    };
}

SeqAccess!()