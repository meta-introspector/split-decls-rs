macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Kind { # [doc = " Returns true if this is a bare repository, one without a work tree."] pub fn is_bare (& self) -> bool { matches ! (self , Kind :: PossiblyBare) } }
    };
}

impl_5!()