// Generated macro for impl_41 (impl)
macro_rules! Depcrate_utils_attr_collectorimpl_41 {
() => {
// Module: crate::utils::attr_collector
// Provides: {"impl_41"}
// Dependencies: {}
impl EarlyLintPass for AttrCollector { fn check_attribute (& mut self , _cx : & EarlyContext < '_ > , attr : & Attribute) { self . attrs . push (attr . span) ; } fn check_crate_post (& mut self , _ : & EarlyContext < '_ > , _ : & Crate) { self . storage . 0 . set (mem :: take (& mut self . attrs)) . expect ("should only be called once") ; } }
};
}
