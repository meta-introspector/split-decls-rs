macro_rules! deps {
    () => {
        EnumAccess!();
        Error!();
        Result!();
        DeserializeSeed!();
        Variant!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < 'de > serde :: de :: EnumAccess < 'de > for & mut (dyn EnumAccess < 'de > + '_) { type Error = Error ; type Variant = Variant < 'de > ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : serde :: de :: DeserializeSeed < 'de > , { let mut erased = erase :: DeserializeSeed :: new (seed) ; match self . erased_variant_seed (& mut erased) { Ok ((out , variant)) => Ok ((unsafe { out . take () } , variant)) , Err (err) => Err (err) , } } }
    };
}

impl_45!();