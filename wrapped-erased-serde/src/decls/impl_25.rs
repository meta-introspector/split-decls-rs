macro_rules! deps {
    () => {
        Error!();
        Deserializer!();
        Out!();
        DeserializeSeed!();
        Result!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'de , T > DeserializeSeed < 'de > for erase :: DeserializeSeed < T > where T : serde :: de :: DeserializeSeed < 'de > , { fn erased_deserialize_seed (& mut self , deserializer : & mut dyn Deserializer < 'de > ,) -> Result < Out , Error > { unsafe { self . take () . deserialize (deserializer) . unsafe_map (Out :: new) } } }
    };
}

impl_25!();