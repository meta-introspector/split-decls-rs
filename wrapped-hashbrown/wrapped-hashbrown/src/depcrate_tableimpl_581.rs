// Generated macro for impl_581 (impl)
macro_rules! Depcrate_tableimpl_581 {
() => {
// Module: crate::table
// Provides: {"impl_581"}
// Dependencies: {}
impl < 'a , T , A > AbsentEntry < 'a , T , A > where A : Allocator , { # [doc = " Converts the `AbsentEntry` into a mutable reference to the underlying"] # [doc = " table."] pub fn into_table (self) -> & 'a mut HashTable < T , A > { self . table } }
};
}
