// Generated macro for truncate (function)
macro_rules! Depcrate_jsontruncate {
() => {
// Module: crate::json
// Provides: {"truncate"}
// Dependencies: {}
# [doc = " Truncates a list to a maximum number of items and prints a message about truncation."] fn truncate < T > (list : & [T] , truncate_after : usize) -> & [T] { if list . len () > truncate_after { println ! ("{} warnings have been truncated for this summary." , list . len () - truncate_after) ; println ! () ; list . split_at (truncate_after) . 0 } else { list } }
};
}
