// Generated macro for impl_55 (impl)
macro_rules! Depcrate_fuzzersimpl_55 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_55"}
// Dependencies: {}
# [doc = " Enables to string and format for `FunctionPrototypeC` types."] impl fmt :: Display for FunctionPrototypeC { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} {} {} func_{}({});" , self . type_qualifier , self . type_name , self . pointer_level , self . ident_id , self . params) } }
};
}
