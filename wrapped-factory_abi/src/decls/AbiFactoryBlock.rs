macro_rules! deps {
    () => {
        FactoryBlockGetCost!();
        FactoryBlockExecute!();
        FactoryBlockGetName!();
    };
}

macro_rules! AbiFactoryBlock {
    () => {
        deps!();
        # [repr (C)] pub struct AbiFactoryBlock { pub block_ptr : * mut c_void , pub get_name : FactoryBlockGetName , pub get_cost : FactoryBlockGetCost , pub execute : FactoryBlockExecute , }
    };
}

AbiFactoryBlock!();