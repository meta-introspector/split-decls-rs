// Generated macro for Id (trait)
macro_rules! Depcrate_build_tableId {
() => {
// Module: crate::build::table
// Provides: {"Id"}
// Dependencies: {}
# [doc = " An identifier for referring to an item in a [`Table`]."] pub trait Id : IdPrivate { # [doc = " Return the index of the item in the table."] fn index (& self) -> usize ; }
};
}
