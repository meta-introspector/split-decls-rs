macro_rules! deps {
    () => {
        GenericBuilder!();
        SCx!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < 'a , 'll , CX : Borrow < SCx < 'll > > > GenericBuilder < 'a , 'll , CX > { fn position_at_start (& mut self , llbb : & 'll BasicBlock) { unsafe { llvm :: LLVMRustPositionBuilderAtStart (self . llbuilder , llbb) ; } } }
    };
}

impl_175!();