// Generated macro for Item (trait)
macro_rules! Depcrate_build_tableItem {
() => {
// Module: crate::build::table
// Provides: {"Item"}
// Dependencies: {}
# [doc = " An item in a [`Table`]."] pub trait Item { # [doc = " The type of identifier for the item."] type Id : Id ; # [doc = " Return `True` if the item is deleted."] fn is_deleted (& self) -> bool ; }
};
}
