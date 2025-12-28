macro_rules! impl_1291 {
    () => {
        impl < FN : Sync + Send + Fn (char) -> bool > Pattern for FN { impl_pattern ! (& self => self) ; }
    };
}

impl_1291!()