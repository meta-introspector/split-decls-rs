macro_rules! deps {
    () => {
        Symbol!();
        FARPROC!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < T > Symbol < T > { # [doc = " Convert the loaded `Symbol` into a handle."] pub fn into_raw (self) -> FARPROC { self . pointer } # [doc = " Convert the loaded `Symbol` into a raw pointer."] pub fn as_raw_ptr (self) -> * mut core :: ffi :: c_void { self . pointer . map (| raw | raw as * mut core :: ffi :: c_void) . unwrap_or (ptr :: null_mut ()) } }
    };
}

impl_115!();