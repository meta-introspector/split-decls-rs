// Generated macro for impl_204 (impl)
macro_rules! Depcrate_collections_stringimpl_204 {
() => {
// Module: crate::collections::string
// Provides: {"impl_204"}
// Dependencies: {}
impl < 'bump > Extend < core_alloc :: string :: String > for String < 'bump > { fn extend < I : IntoIterator < Item = core_alloc :: string :: String > > (& mut self , iter : I) { for s in iter { self . push_str (& s) } } }
};
}
