// Generated macro for char_repeat (function)
macro_rules! Depcrate_borrow_tracker_tree_borrows_diagnosticschar_repeat {
() => {
// Module: crate::borrow_tracker::tree_borrows::diagnostics
// Provides: {"char_repeat"}
// Dependencies: {}
# [doc = " Repeat a character a number of times."] fn char_repeat (c : char , n : usize) -> String { std :: iter :: once (c) . cycle () . take (n) . collect :: < String > () }
};
}
