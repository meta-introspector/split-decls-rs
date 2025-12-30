// Generated macro for impl_43 (impl)
macro_rules! Depcrate_fuzzersimpl_43 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_43"}
// Dependencies: {}
# [doc = " Enables to string and format for `BasicTypeDeclarationC` types."] impl fmt :: Display for BasicTypeDeclarationC { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} {} {} ident_{}{};" , self . type_qualifier , self . type_name , self . pointer_level , self . ident_id , self . array_dimension) } }
};
}
