// Generated macro for impl_61 (impl)
macro_rules! Depcrate_typesimpl_61 {
() => {
// Module: crate::types
// Provides: {"impl_61"}
// Dependencies: {}
impl Display for BaseType { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match self { Self :: Named (name) => f . write_str (name) , Self :: List (ty) => write ! (f , "[{}]" , ty) , } } }
};
}
