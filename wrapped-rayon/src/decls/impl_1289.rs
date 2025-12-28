macro_rules! impl_1289 {
    () => {
        impl < const N : usize > Pattern for [char ; N] { impl_pattern ! (& self => * self) ; }
    };
}

impl_1289!();