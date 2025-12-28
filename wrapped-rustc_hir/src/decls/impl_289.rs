macro_rules! deps {
    () => {
        ImplicitSelfKind!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl ImplicitSelfKind { # [doc = " Does this represent an implicit self?"] pub fn has_implicit_self (& self) -> bool { ! matches ! (* self , ImplicitSelfKind :: None) } }
    };
}

impl_289!();