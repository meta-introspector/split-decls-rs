// Generated macro for impl_113 (impl)
macro_rules! Depcrate_mockable_itemimpl_113 {
() => {
// Module: crate::mockable_item
// Provides: {"impl_113"}
// Dependencies: {}
impl From < (Attrs , Item) > for MockableItem { fn from ((attrs , item) : (Attrs , Item)) -> MockableItem { match item { Item :: Impl (item_impl) => MockableItem :: Struct (MockableStruct :: from (item_impl)) , Item :: Mod (item_mod) => MockableItem :: Module (MockableModule :: from (item_mod)) , Item :: Trait (trait_) => MockableItem :: Struct (MockableStruct :: from ((attrs , trait_))) , _ => panic ! ("automock does not support this item type") } } }
};
}
