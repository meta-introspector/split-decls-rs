macro_rules! deps {
    () => {
        SCx!();
        GenericCx!();
    };
}

macro_rules! impl_370 {
    () => {
        deps!();
        impl < 'll , CX : Borrow < SCx < 'll > > > GenericCx < 'll , CX > { # [doc = " Declare a global value."] # [doc = ""] # [doc = " If there’s a value with the same name already declared, the function will"] # [doc = " return its Value instead."] pub (crate) fn declare_global (& self , name : & str , ty : & 'll Type) -> & 'll Value { debug ! ("declare_global(name={:?})" , name) ; unsafe { llvm :: LLVMRustGetOrInsertGlobal ((* * self) . borrow () . llmod , name . as_c_char_ptr () , name . len () , ty ,) } } }
    };
}

impl_370!()