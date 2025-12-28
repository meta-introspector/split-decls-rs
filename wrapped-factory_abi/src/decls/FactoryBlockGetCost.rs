macro_rules! FactoryBlockGetCost {
    () => {
        pub type FactoryBlockGetCost = extern "C" fn (block_ptr : * mut c_void) -> u32 ;
    };
}

FactoryBlockGetCost!()