macro_rules! deps {
    () => {
        ToLlvmBool!();
        Bool!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl ToLlvmBool for bool { # [inline (always)] fn to_llvm_bool (self) -> llvm :: Bool { llvm :: Bool :: from_bool (self) } }
    };
}

impl_427!()