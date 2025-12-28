macro_rules! deps {
    () => {
        Deserializer!();
        Out!();
        Result!();
        Error!();
    };
}

macro_rules! DeserializeSeed {
    () => {
        deps!();
        pub trait DeserializeSeed < 'de > { fn erased_deserialize_seed (& mut self , deserializer : & mut dyn Deserializer < 'de > ,) -> Result < Out , Error > ; }
    };
}

DeserializeSeed!()