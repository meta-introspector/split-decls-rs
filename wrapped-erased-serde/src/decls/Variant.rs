macro_rules! deps {
    () => {
        DeserializeSeed!();
        Any!();
        Result!();
        Visitor!();
        Out!();
        Error!();
    };
}

macro_rules! Variant {
    () => {
        deps!();
        pub struct Variant < 'de > { data : Any , unit_variant : unsafe fn (Any) -> Result < () , Error > , visit_newtype : unsafe fn (Any , seed : & mut dyn DeserializeSeed < 'de >) -> Result < Out , Error > , tuple_variant : unsafe fn (Any , len : usize , visitor : & mut dyn Visitor < 'de >) -> Result < Out , Error > , struct_variant : unsafe fn (Any , fields : & 'static [& 'static str] , visitor : & mut dyn Visitor < 'de > ,) -> Result < Out , Error > , }
    };
}

Variant!();