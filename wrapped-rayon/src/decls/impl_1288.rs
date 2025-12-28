macro_rules! impl_1288 {
    () => {
        impl Pattern for & [char] { impl_pattern ! (& self => * self) ; }
    };
}

impl_1288!();