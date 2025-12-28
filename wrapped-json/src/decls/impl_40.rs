macro_rules! deps {
    () => {
        Error!();
        Result!();
        Value!();
        UnitVariantAccess!();
        Read!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < 'de , 'a , R : Read < 'de > + 'a > de :: EnumAccess < 'de > for UnitVariantAccess < 'a , R > { type Error = Error ; type Variant = Self ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self) > where V : de :: DeserializeSeed < 'de > , { let variant = tri ! (seed . deserialize (& mut * self . de)) ; Ok ((variant , self)) } }
    };
}

impl_40!();