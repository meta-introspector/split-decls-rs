macro_rules! deps {
    () => {
        InPlaceSeed!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'a , 'de , T > serde :: de :: DeserializeSeed < 'de > for InPlaceSeed < 'a , T > where T : serde :: de :: Deserialize < 'de > , { type Value = () ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { T :: deserialize_in_place (deserializer , self . 0) } }
    };
}

impl_197!();