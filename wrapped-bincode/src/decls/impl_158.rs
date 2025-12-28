macro_rules! deps {
    () => {
        Decoder!();
        SerdeDecoder!();
        DecodeError!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < 'de , DE : Decoder > EnumAccess < 'de > for SerdeDecoder < '_ , DE > { type Error = DecodeError ; type Variant = Self ; fn variant_seed < V > (mut self , seed : V) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : DeserializeSeed < 'de > , { let idx = u32 :: decode (& mut self . de) ? ; let val = seed . deserialize (idx . into_deserializer ()) ? ; Ok ((val , self)) } }
    };
}

impl_158!();