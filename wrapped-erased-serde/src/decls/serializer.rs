macro_rules! serializer {
    () => {
        pub mod serializer { pub trait Sealed { } }
    };
}

serializer!();