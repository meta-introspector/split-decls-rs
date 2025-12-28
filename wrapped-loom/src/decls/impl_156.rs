macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl Id { # [doc = " Returns an integer ID unique to this current execution (for use in"] # [doc = " [`thread::ThreadId`]'s `Debug` impl)"] pub (crate) fn public_id (& self) -> usize { self . id } }
    };
}

impl_156!()