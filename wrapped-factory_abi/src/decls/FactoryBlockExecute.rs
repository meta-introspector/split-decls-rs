macro_rules! deps {
    () => {
        FactoryContext!();
    };
}

macro_rules! FactoryBlockExecute {
    () => {
        deps!();
        pub type FactoryBlockExecute = extern "C" fn (block_ptr : * mut c_void , factory_ctx : FactoryContext , crate_path : * const c_char ,) -> bool ;
    };
}

FactoryBlockExecute!();