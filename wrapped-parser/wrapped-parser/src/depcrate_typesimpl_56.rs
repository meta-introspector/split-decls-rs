// Generated macro for impl_56 (impl)
macro_rules! Depcrate_typesimpl_56 {
() => {
// Module: crate::types
// Provides: {"impl_56"}
// Dependencies: {}
impl Display for OperationType { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (match self { Self :: Query => "query" , Self :: Mutation => "mutation" , Self :: Subscription => "subscription" , }) } }
};
}
