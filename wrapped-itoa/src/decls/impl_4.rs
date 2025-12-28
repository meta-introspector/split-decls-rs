macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        # [allow (clippy :: non_canonical_clone_impl)] impl Clone for Buffer { # [inline] fn clone (& self) -> Self { Buffer :: new () } }
    };
}

impl_4!()