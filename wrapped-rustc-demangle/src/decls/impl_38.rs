macro_rules! deps {
    () => {
        Demangle!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < 'a > Demangle < 'a > { # [doc = " Returns the underlying string that's being demangled."] pub fn as_str (& self) -> & 'a str { self . original } }
    };
}

impl_38!();