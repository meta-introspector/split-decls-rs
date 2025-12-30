// Generated macro for impl_423 (impl)
macro_rules! Depcrate_iter_collect_consumerimpl_423 {
() => {
// Module: crate::iter::collect::consumer
// Provides: {"impl_423"}
// Dependencies: {}
impl < T : Send > CollectConsumer < '_ , T > { # [doc = " Create a collector for `len` items in the unused capacity of the vector."] pub (super) fn appender (vec : & mut Vec < T > , len : usize) -> CollectConsumer < '_ , T > { let start = vec . len () ; assert ! (vec . capacity () - start >= len) ; unsafe { CollectConsumer :: new (vec . as_mut_ptr () . add (start) , len) } } }
};
}
