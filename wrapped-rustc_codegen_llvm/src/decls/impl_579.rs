macro_rules! impl_579 {
    () => {
        impl fmt :: Debug for Type { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (& llvm :: build_string (| s | unsafe { llvm :: LLVMRustWriteTypeToString (self , s) ; }) . expect ("non-UTF8 type description from LLVM") ,) } }
    };
}

impl_579!()