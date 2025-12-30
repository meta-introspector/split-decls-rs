// Generated macro for DisplayFmtPermission (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsDisplayFmtPermission {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"DisplayFmtPermission"}
// Dependencies: {}
# [doc = " Formatting of the permissions on each range."] # [doc = ""] # [doc = " Example:"] # [doc = " ```rust,ignore (private type)"] # [doc = " DisplayFmtPermission {"] # [doc = "     open: \"[\","] # [doc = "     sep: \"|\","] # [doc = "     close: \"]\","] # [doc = "     uninit: \"___\","] # [doc = "     range_sep: \"..\","] # [doc = " }"] # [doc = " ```"] # [doc = " will show each permission line as"] # [doc = " ```text"] # [doc = " 0.. 1.. 2.. 3.. 4.. 5"] # [doc = " [Act|Res|Frz|Dis|___]"] # [doc = " ```"] struct DisplayFmtPermission { # [doc = " Text that starts the permission block."] open : S , # [doc = " Text that separates permissions on different ranges."] sep : S , # [doc = " Text that ends the permission block."] close : S , # [doc = " Text to show when a permission is not initialized."] # [doc = " Should have the same width as a `Permission`'s `.short_name()`, i.e."] # [doc = " 3 if using the `Res/Act/Frz/Dis` notation."] uninit : S , # [doc = " Text to separate the `start` and `end` values of a range."] range_sep : S , }
};
}
