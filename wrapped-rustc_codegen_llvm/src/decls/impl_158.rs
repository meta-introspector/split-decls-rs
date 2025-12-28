macro_rules! deps {
    () => {
        GenericBuilder!();
        SCx!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < 'a , 'll , CX : Borrow < SCx < 'll > > > Drop for GenericBuilder < 'a , 'll , CX > { fn drop (& mut self) { unsafe { llvm :: LLVMDisposeBuilder (& mut * (self . llbuilder as * mut _)) ; } } }
    };
}

impl_158!()