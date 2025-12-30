// Generated macro for impl_944 (impl)
macro_rules! Depcrate_tz_concatenatedimpl_944 {
() => {
// Module: crate::tz::concatenated
// Provides: {"impl_944"}
// Dependencies: {}
impl < 'a > core :: fmt :: Debug for IndexEntry < 'a > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_struct ("IndexEntry") . field ("name" , & escape :: Bytes (self . name_bytes ())) . field ("start" , & self . start ()) . field ("len" , & self . len ()) . finish () } }
};
}
