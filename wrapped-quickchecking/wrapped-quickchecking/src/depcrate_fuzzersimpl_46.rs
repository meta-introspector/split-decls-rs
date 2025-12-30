// Generated macro for impl_46 (impl)
macro_rules! Depcrate_fuzzersimpl_46 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_46"}
// Dependencies: {}
# [doc = " Enables to string and format for `StructDeclarationC` types."] impl fmt :: Display for StructDeclarationC { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "struct {{ {} }} struct_{}{};" , self . fields , self . ident_id , self . array_dimension) } }
};
}
