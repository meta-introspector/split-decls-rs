macro_rules! deps {
    () => {
        SCx!();
        GenericBuilder!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < 'a , 'll , CX : Borrow < SCx < 'll > > > GenericBuilder < 'a , 'll , CX > { pub (crate) fn phi (& mut self , ty : & 'll Type , vals : & [& 'll Value] , bbs : & [& 'll BasicBlock] ,) -> & 'll Value { assert_eq ! (vals . len () , bbs . len ()) ; let phi = unsafe { llvm :: LLVMBuildPhi (self . llbuilder , ty , UNNAMED) } ; unsafe { llvm :: LLVMAddIncoming (phi , vals . as_ptr () , bbs . as_ptr () , vals . len () as c_uint) ; phi } } fn add_incoming_to_phi (& mut self , phi : & 'll Value , val : & 'll Value , bb : & 'll BasicBlock) { unsafe { llvm :: LLVMAddIncoming (phi , & val , & bb , 1 as c_uint) ; } } }
    };
}

impl_179!();