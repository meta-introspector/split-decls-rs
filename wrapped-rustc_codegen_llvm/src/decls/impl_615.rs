macro_rules! impl_615 {
    () => {
        impl fmt :: Debug for Value { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (& llvm :: build_string (| s | unsafe { llvm :: LLVMRustWriteValueToString (self , s) ; }) . expect ("non-UTF8 value description from LLVM") ,) } }
    };
}

impl_615!();