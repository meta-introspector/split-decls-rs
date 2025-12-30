// Generated macro for impl_52 (impl)
macro_rules! Depcrate_fuzzersimpl_52 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_52"}
// Dependencies: {}
# [doc = " Enables to string and format for `FunctionPointerDeclarationC` types."] impl fmt :: Display for FunctionPointerDeclarationC { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} {} {} (*func_ptr_{})({});" , self . type_qualifier , self . type_name , self . pointer_level , self . ident_id , self . params) } }
};
}
