macro_rules! GetSymbolsErrorCallback {
    () => {
        pub (crate) type GetSymbolsErrorCallback = unsafe extern "C" fn (* const c_char) -> * mut c_void ;
    };
}

GetSymbolsErrorCallback!();