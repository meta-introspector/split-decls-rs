// Generated macro for Permission (enum)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_itemPermission {
() => {
// Module: crate::borrow_tracker::stacked_borrows::item
// Provides: {"Permission"}
// Dependencies: {}
# [doc = " Indicates which permission is granted (by this item to some pointers)"] # [derive (Copy , Clone , Debug , Hash , PartialEq , Eq)] pub enum Permission { # [doc = " Grants unique mutable access."] Unique , # [doc = " Grants shared mutable access."] SharedReadWrite , # [doc = " Grants shared read-only access."] SharedReadOnly , # [doc = " Grants no access, but separates two groups of SharedReadWrite so they are not"] # [doc = " all considered mutually compatible."] Disabled , }
};
}
