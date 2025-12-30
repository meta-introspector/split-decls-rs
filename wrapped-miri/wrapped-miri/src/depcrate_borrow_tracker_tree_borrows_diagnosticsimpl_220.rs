// Generated macro for impl_220 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticsimpl_220 {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"impl_220"}
// Dependencies: {}
impl DisplayIndent { fn new () -> Self { Self { curr : "    " . to_string () } } # [doc = " Increment the indentation by one. Note: need to know if this"] # [doc = " is the last child or not because the presence of other children"] # [doc = " changes the way the indentation is shown."] fn increment (& mut self , formatter : & DisplayFmt , is_last : bool) { self . curr . push_str (if is_last { formatter . padding . indent_last } else { formatter . padding . indent_middle }) ; } # [doc = " Pop the last level of indentation."] fn decrement (& mut self , formatter : & DisplayFmt) { for _ in 0 .. formatter . padding . indent_last . len () { let _ = self . curr . pop () ; } } # [doc = " Print the current indentation."] fn write (& self , s : & mut String) { s . push_str (& self . curr) ; } }
};
}
