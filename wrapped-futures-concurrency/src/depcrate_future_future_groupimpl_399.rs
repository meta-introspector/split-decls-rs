// Generated macro for impl_399 (impl)
macro_rules! Depcrate_future_future_groupimpl_399 {
() => {
// Module: crate::future::future_group
// Provides: {"impl_399"}
// Dependencies: {}
impl < T : Debug > Debug for FutureGroup < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FutureGroup") . field ("slab" , & "[..]") . field ("len" , & self . len ()) . field ("capacity" , & self . capacity) . finish () } }
};
}
