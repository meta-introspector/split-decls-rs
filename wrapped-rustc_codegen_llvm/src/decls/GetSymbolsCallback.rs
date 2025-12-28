macro_rules! GetSymbolsCallback {
    () => {
        pub (crate) type GetSymbolsCallback = unsafe extern "C" fn (* mut c_void , * const c_char) -> * mut c_void ;
    };
}

GetSymbolsCallback!();