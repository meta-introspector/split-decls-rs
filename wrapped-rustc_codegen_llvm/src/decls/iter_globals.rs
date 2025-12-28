macro_rules! deps {
    () => {
        ValueIter!();
    };
}

macro_rules! iter_globals {
    () => {
        deps!();
        pub (crate) fn iter_globals (llmod : & llvm :: Module) -> ValueIter < '_ > { unsafe { ValueIter { cur : llvm :: LLVMGetFirstGlobal (llmod) , step : llvm :: LLVMGetNextGlobal } } }
    };
}

iter_globals!()