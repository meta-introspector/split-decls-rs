macro_rules! deps {
    () => {
        Read!();
        Value!();
        Result!();
        VariantAccess!();
        Error!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < 'de , 'a , R : Read < 'de > + 'a > de :: EnumAccess < 'de > for VariantAccess < 'a , R > { type Error = Error ; type Variant = Self ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self) > where V : de :: DeserializeSeed < 'de > , { let val = tri ! (seed . deserialize (& mut * self . de)) ; tri ! (self . de . parse_object_colon ()) ; Ok ((val , self)) } }
    };
}

impl_36!();