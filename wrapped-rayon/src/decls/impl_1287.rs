macro_rules! impl_1287 {
    () => {
        impl Pattern for char { impl_pattern ! (& self => * self) ; }
    };
}

impl_1287!()