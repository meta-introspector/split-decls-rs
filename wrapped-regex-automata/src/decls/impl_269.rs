macro_rules! deps {
    () => {
        LazyStateIDError!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl LazyStateIDError { # [doc = " Returns the value that failed to constructed a lazy state ID."] pub (crate) fn attempted (& self) -> u64 { self . attempted } }
    };
}

impl_269!()