macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Clone for Box < Utf8Path > { fn clone (& self) -> Self { let boxed : Box < Path > = self . 0 . into () ; let ptr = Box :: into_raw (boxed) as * mut Utf8Path ; unsafe { Box :: from_raw (ptr) } } }
    };
}

impl_29!()