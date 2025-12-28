macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_362 {
    () => {
        deps!();
        impl < 'll > Builder < '_ , 'll , '_ > { pub (crate) fn get_dbg_loc (& self) -> Option < & 'll DILocation > { unsafe { llvm :: LLVMGetCurrentDebugLocation2 (self . llbuilder) } } }
    };
}

impl_362!();