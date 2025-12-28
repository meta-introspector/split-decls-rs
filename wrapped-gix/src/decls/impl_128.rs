macro_rules! deps {
    () => {
        Kind!();
        Repository!();
        Head!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl Kind { # [doc = " Attach this instance to a `repo` to produce a [`Head`]."] pub fn attach (self , repo : & crate :: Repository) -> Head < '_ > { Head { kind : self , repo } } }
    };
}

impl_128!()