macro_rules! deps {
    () => {
        Read!();
        Result!();
        Value!();
        Error!();
        VariantAccess!();
        Deserializer!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < 'de , 'a , R : Read < 'de > + 'a > de :: VariantAccess < 'de > for VariantAccess < 'a , R > { type Error = Error ; fn unit_variant (self) -> Result < () > { de :: Deserialize :: deserialize (self . de) } fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value > where T : de :: DeserializeSeed < 'de > , { seed . deserialize (self . de) } fn tuple_variant < V > (self , _len : usize , visitor : V) -> Result < V :: Value > where V : de :: Visitor < 'de > , { de :: Deserializer :: deserialize_seq (self . de , visitor) } fn struct_variant < V > (self , fields : & 'static [& 'static str] , visitor : V) -> Result < V :: Value > where V : de :: Visitor < 'de > , { de :: Deserializer :: deserialize_struct (self . de , "" , fields , visitor) } }
    };
}

impl_37!();