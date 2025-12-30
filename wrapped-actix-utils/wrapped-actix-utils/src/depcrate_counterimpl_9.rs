// Generated macro for impl_9 (impl)
macro_rules! Depcrate_counterimpl_9 {
() => {
// Module: crate::counter
// Provides: {"impl_9"}
// Dependencies: {}
impl fmt :: Debug for CounterInner { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Counter") . field ("count" , & self . count . get ()) . field ("capacity" , & self . capacity) . field ("task" , & self . task) . finish () } }
};
}
