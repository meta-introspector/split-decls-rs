macro_rules! deps {
    () => {
        Options!();
        Root!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Options { # [doc = " Create a new [`Root`](./tree/struct.Root.html) instance from the"] # [doc = " configuration within."] pub fn create (self) -> Root { self . into () } }
    };
}

impl_14!();