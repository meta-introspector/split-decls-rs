macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl < 'a , 'll , 'tcx > Builder < 'a , 'll , 'tcx > { pub (crate) fn llfn (& self) -> & 'll Value { unsafe { llvm :: LLVMGetBasicBlockParent (self . llbb ()) } } }
    };
}

impl_174!()