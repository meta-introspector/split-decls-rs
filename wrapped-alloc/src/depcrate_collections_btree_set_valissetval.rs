// Generated macro for IsSetVal (trait)
macro_rules! Depcrate_collections_btree_set_valIsSetVal {
() => {
// Module: crate::collections::btree::set_val
// Provides: {"IsSetVal"}
// Dependencies: {}
# [doc = " A trait to differentiate between `BTreeMap` and `BTreeSet` values."] # [doc = " Returns `true` only for type `SetValZST`, `false` for all other types (blanket implementation)."] # [doc = " `TypeId` requires a `'static` lifetime, use of this trait avoids that restriction."] # [doc = ""] # [doc = " [`TypeId`]: core::any::TypeId"] pub (super) trait IsSetVal { fn is_set_val () -> bool ; }
};
}
