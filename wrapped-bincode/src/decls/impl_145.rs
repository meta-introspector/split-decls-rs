macro_rules! deps {
    () => {
        DecodeError!();
        SerdeDecoder!();
        BorrowDecoder!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < 'de , DE : BorrowDecoder < 'de > > EnumAccess < 'de > for SerdeDecoder < '_ , 'de , DE > { type Error = DecodeError ; type Variant = Self ; fn variant_seed < V > (mut self , seed : V) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : DeserializeSeed < 'de > , { let idx = u32 :: decode (& mut self . de) ? ; let val = seed . deserialize (idx . into_deserializer ()) ? ; Ok ((val , self)) } }
    };
}

impl_145!();