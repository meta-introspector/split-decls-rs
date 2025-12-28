macro_rules! deps {
    () => {
        SimpleCx!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl < 'll > SimpleCx < 'll > { pub (crate) fn get_type_of_global (& self , val : & 'll Value) -> & 'll Type { unsafe { llvm :: LLVMGlobalGetValueType (val) } } pub (crate) fn val_ty (& self , v : & 'll Value) -> & 'll Type { common :: val_ty (v) } }
    };
}

impl_219!()