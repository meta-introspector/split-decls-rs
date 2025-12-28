macro_rules! deps {
    () => {
        Out!();
        Error!();
        Deserializer!();
        Result!();
    };
}

macro_rules! DeserializeSeed {
    () => {
        deps!();
        pub trait DeserializeSeed < 'de > { fn erased_deserialize_seed (& mut self , deserializer : & mut dyn Deserializer < 'de > ,) -> Result < Out , Error > ; }
    };
}

DeserializeSeed!();