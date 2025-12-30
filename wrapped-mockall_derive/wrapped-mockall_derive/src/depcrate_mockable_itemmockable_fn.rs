// Generated macro for mockable_fn (function)
macro_rules! Depcrate_mockable_itemmockable_fn {
() => {
// Module: crate::mockable_item
// Provides: {"mockable_fn"}
// Dependencies: {}
# [doc = " Performs transformations on a function to make it mockable"] fn mockable_fn (mut item_fn : ItemFn) -> ItemFn { demutify (& mut item_fn . sig . inputs) ; deimplify (& mut item_fn . sig . output) ; item_fn }
};
}
