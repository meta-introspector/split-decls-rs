macro_rules! deserializer {
    () => {
        pub mod deserializer { pub trait Sealed { } }
    };
}

deserializer!();