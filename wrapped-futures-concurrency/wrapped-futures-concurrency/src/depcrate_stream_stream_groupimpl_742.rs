// Generated macro for impl_742 (impl)
macro_rules! Depcrate_stream_stream_groupimpl_742 {
() => {
// Module: crate::stream::stream_group
// Provides: {"impl_742"}
// Dependencies: {}
impl < T : Debug > Debug for StreamGroup < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("StreamGroup") . field ("slab" , & "[..]") . finish () } }
};
}
