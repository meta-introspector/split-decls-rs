// Generated macro for impl_344 (impl)
macro_rules! Depcrate_tabsimpl_344 {
() => {
// Module: crate::tabs
// Provides: {"impl_344"}
// Dependencies: {}
impl < 'a , Item > FromIterator < Item > for Tabs < 'a > where Item : Into < Line < 'a > > , { fn from_iter < Iter : IntoIterator < Item = Item > > (iter : Iter) -> Self { Self :: new (iter) } }
};
}
