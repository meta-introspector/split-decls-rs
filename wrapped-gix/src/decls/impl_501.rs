macro_rules! deps {
    () => {
        Options!();
        Path!();
    };
}

macro_rules! impl_501 {
    () => {
        deps!();
        impl Options { pub (crate) fn current_dir_or_empty (& self) -> & std :: path :: Path { self . current_dir . as_deref () . unwrap_or (std :: path :: Path :: new ("")) } }
    };
}

impl_501!()