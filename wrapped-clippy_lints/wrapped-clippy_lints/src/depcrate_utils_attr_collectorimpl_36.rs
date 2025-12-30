// Generated macro for impl_36 (impl)
macro_rules! Depcrate_utils_attr_collectorimpl_36 {
() => {
// Module: crate::utils::attr_collector
// Provides: {"impl_36"}
// Dependencies: {}
impl EarlyLintPass for AttrCollector { fn check_attribute (& mut self , _cx : & EarlyContext < '_ > , attr : & Attribute) { self . attrs . push (attr . span) ; } fn check_crate_post (& mut self , _ : & EarlyContext < '_ > , _ : & Crate) { self . storage . 0 . set (mem :: take (& mut self . attrs)) . expect ("should only be called once") ; } }
};
}
