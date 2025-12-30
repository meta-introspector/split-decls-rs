// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
impl core :: fmt :: Debug for ExportDriver { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("ExportDriver") . field ("markers" , & self . markers) . field ("requested_families" , & self . requested_families) . field ("attributes_filters" , & self . attributes_filters . keys ()) . field ("fallbacker" , & self . fallbacker) . field ("include_full" , & self . include_full) . field ("deduplication_strategy" , & self . deduplication_strategy) . finish () } }
};
}
