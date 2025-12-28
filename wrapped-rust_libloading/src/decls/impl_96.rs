macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < T > Symbol < T > { # [doc = " Convert the loaded `Symbol` into a raw pointer."] pub fn into_raw (self) -> * mut core :: ffi :: c_void { self . pointer } # [doc = " Convert the loaded `Symbol` into a raw pointer."] # [doc = " For unix this does the same as into_raw."] pub fn as_raw_ptr (self) -> * mut core :: ffi :: c_void { self . pointer } }
    };
}

impl_96!()