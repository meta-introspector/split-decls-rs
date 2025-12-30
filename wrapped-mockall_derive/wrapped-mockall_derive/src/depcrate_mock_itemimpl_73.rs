// Generated macro for impl_73 (impl)
macro_rules! Depcrate_mock_itemimpl_73 {
() => {
// Module: crate::mock_item
// Provides: {"impl_73"}
// Dependencies: {}
impl From < MockableItem > for MockItem { fn from (mockable : MockableItem) -> MockItem { match mockable { MockableItem :: Struct (s) => MockItem :: Struct (MockItemStruct :: from (s)) , MockableItem :: Module (mod_) => MockItem :: Module (MockItemModule :: from (mod_)) } } }
};
}
