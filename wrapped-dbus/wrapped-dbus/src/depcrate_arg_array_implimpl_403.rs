// Generated macro for impl_403 (impl)
macro_rules! Depcrate_arg_array_implimpl_403 {
() => {
// Module: crate::arg::array_impl
// Provides: {"impl_403"}
// Dependencies: {}
impl < 'a , K : DictKey , V : Arg , I > Dict < 'a , K , V , I > { fn entry_sig () -> String { format ! ("{{{}{}}}" , K :: signature () , V :: signature ()) } }
};
}
