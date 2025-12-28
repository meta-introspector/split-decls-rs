macro_rules! deps {
    () => {
        DeserializeSeed!();
        Result!();
        Out!();
        Variant!();
        Error!();
    };
}

macro_rules! EnumAccess {
    () => {
        deps!();
        pub trait EnumAccess < 'de > { fn erased_variant_seed (& mut self , seed : & mut dyn DeserializeSeed < 'de > ,) -> Result < (Out , Variant < 'de >) , Error > ; }
    };
}

EnumAccess!()