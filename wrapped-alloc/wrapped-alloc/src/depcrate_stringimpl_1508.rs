// Generated macro for impl_1508 (impl)
macro_rules! Depcrate_stringimpl_1508 {
() => {
// Module: crate::string
// Provides: {"impl_1508"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl fmt :: Debug for Drain < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Drain") . field (& self . as_str ()) . finish () } }
};
}
