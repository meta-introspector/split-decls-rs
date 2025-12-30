// Generated macro for impl_53 (impl)
macro_rules! Depcrate_fuzzersimpl_53 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_53"}
// Dependencies: {}
# [doc = " `MakeUnique` is used in generation of C headers to make `FunctionPrototypeC`"] # [doc = " identifiers unique."] impl MakeUnique for FunctionPrototypeC { fn make_unique (& mut self , stamp : usize) { let _ = write ! (self . ident_id , "_{stamp}") ; } }
};
}
