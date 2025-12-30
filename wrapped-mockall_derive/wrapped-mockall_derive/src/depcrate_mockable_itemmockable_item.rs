// Generated macro for mockable_item (function)
macro_rules! Depcrate_mockable_itemmockable_item {
() => {
// Module: crate::mockable_item
// Provides: {"mockable_item"}
// Dependencies: {}
# [doc = " Performs transformations on an Item to make it mockable"] fn mockable_item (item : Item) -> Item { match item { Item :: Fn (item_fn) => Item :: Fn (mockable_fn (item_fn)) , Item :: ForeignMod (ifm) => Item :: ForeignMod (mockable_foreign_mod (ifm)) , x => x } }
};
}
