macro_rules! deps {
    () => {
        AtomicBool!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        impl Default for AtomicBool { fn default () -> AtomicBool { AtomicBool :: new (Default :: default ()) } }
    };
}

impl_254!()