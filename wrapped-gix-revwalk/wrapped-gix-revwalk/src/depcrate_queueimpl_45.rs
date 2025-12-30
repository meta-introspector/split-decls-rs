// Generated macro for impl_45 (impl)
macro_rules! Depcrate_queueimpl_45 {
() => {
// Module: crate::queue
// Provides: {"impl_45"}
// Dependencies: {}
impl < K : Ord , T > PartialOrd < Self > for Item < K , T > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (Ord :: cmp (self , other)) } }
};
}
