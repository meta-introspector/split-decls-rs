// Generated macro for impl_445 (impl)
macro_rules! Depcrate_rustc_entryimpl_445 {
() => {
// Module: crate::rustc_entry
// Provides: {"impl_445"}
// Dependencies: {}
impl < K : Debug , V : Debug , A : Allocator > Debug for RustcEntry < '_ , K , V , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Vacant (ref v) => f . debug_tuple ("Entry") . field (v) . finish () , Occupied (ref o) => f . debug_tuple ("Entry") . field (o) . finish () , } } }
};
}
