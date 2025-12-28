macro_rules! FactoryBlockGetName {
    () => {
        pub type FactoryBlockGetName = extern "C" fn (block_ptr : * mut c_void) -> * const c_char ;
    };
}

FactoryBlockGetName!();