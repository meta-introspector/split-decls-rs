macro_rules! deps {
    () => {
        Bool!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        impl Bool { pub (crate) const TRUE : Self = Self { value : 1 } ; pub (crate) const FALSE : Self = Self { value : 0 } ; pub (crate) const fn from_bool (rust_bool : bool) -> Self { if rust_bool { Self :: TRUE } else { Self :: FALSE } } # [doc = " Converts this LLVM-C boolean to a Rust `bool`"] pub (crate) fn is_true (self) -> bool { self . value != Self :: FALSE . value } }
    };
}

impl_424!()