// Generated macro for impl_49 (impl)
macro_rules! Depcrate_fuzzersimpl_49 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_49"}
// Dependencies: {}
# [doc = " Enables to string and format for `UnionDeclarationC` types."] impl fmt :: Display for UnionDeclarationC { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "union {{ {} }} union_{}{};" , self . fields , self . ident_id , self . array_dimension) } }
};
}
