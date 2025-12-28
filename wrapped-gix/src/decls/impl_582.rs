macro_rules! deps {
    () => {
        Author!();
        Key!();
        Section!();
    };
}

macro_rules! impl_582 {
    () => {
        deps!();
        impl Section for Author { fn name (& self) -> & str { "author" } fn keys (& self) -> & [& dyn Key] { & [& Self :: NAME , & Self :: EMAIL] } }
    };
}

impl_582!()