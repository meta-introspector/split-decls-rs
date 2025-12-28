macro_rules! deps {
    () => {
        Unparker!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl Clone for Unparker { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } }
    };
}

impl_104!();