macro_rules! MemoryEffects {
    () => {
        # [doc = " LLVMRustMemoryEffects"] # [derive (Copy , Clone)] # [repr (C)] pub (crate) enum MemoryEffects { None , ReadOnly , InaccessibleMemOnly , }
    };
}

MemoryEffects!();