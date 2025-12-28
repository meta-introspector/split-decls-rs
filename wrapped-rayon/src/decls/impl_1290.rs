macro_rules! impl_1290 {
    () => {
        impl < const N : usize > Pattern for & [char ; N] { impl_pattern ! (& self => * self) ; }
    };
}

impl_1290!()