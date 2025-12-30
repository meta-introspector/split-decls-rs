// Generated macro for impl_893 (impl)
macro_rules! Depcrate_util_any_valueimpl_893 {
() => {
// Module: crate::util::any_value
// Provides: {"impl_893"}
// Dependencies: {}
impl std :: fmt :: Debug for AnyValue { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . debug_struct ("AnyValue") . field ("inner" , & self . id) . finish () } }
};
}
