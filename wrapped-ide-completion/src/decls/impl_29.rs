macro_rules! deps {
    () => {
        Completions!();
        Builder!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Builder { # [doc = " Convenience method, which allows to add a freshly created completion into accumulator"] # [doc = " without binding it to the variable."] pub (crate) fn add_to (self , acc : & mut Completions , db : & RootDatabase) { acc . add (self . build (db)) } }
    };
}

impl_29!()