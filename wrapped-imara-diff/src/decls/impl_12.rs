macro_rules! deps {
    () => {
        InternedInput!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < T > InternedInput < T > { pub fn clear (& mut self) { self . before . clear () ; self . after . clear () ; self . interner . clear () ; } }
    };
}

impl_12!();