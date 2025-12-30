// Generated macro for standard_pointer_impls (macro)
macro_rules! Depcrate_runtimestandard_pointer_impls {
() => {
// Module: crate::runtime
// Provides: {"standard_pointer_impls"}
// Dependencies: {}
# [doc = " Implement PartialEq, Eq and Hash using pointer semantics; there's not"] # [doc = " really a better way to do it for this type"] macro_rules ! standard_pointer_impls { ($ name : ident) => { impl PartialEq for $ name { # [inline] fn eq (& self , other : & Self) -> bool { ptr :: eq (self , other) } } impl Eq for $ name { } impl hash :: Hash for $ name { # [inline] fn hash < H : hash :: Hasher > (& self , state : & mut H) { let ptr : * const Self = self ; ptr . hash (state) } } } ; }
};
}
