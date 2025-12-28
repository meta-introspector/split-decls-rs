macro_rules! deps {
    () => {
        Secret!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T : AsRef < str > > Secret < T > { # [doc = " Checks if the contained value is empty."] pub fn is_empty (& self) -> bool { self . inner . as_ref () . is_empty () } }
    };
}

impl_15!();