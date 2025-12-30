// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'ast > Visit < 'ast > for ImportVisitor { fn visit_item_use (& mut self , node : & 'ast syn :: ItemUse) { self . imports . push (quote :: quote ! (# node) . to_string ()) ; } }
};
}
