// Generated macro for borrow_of_local_data (function)
macro_rules! Depcrate_path_utilsborrow_of_local_data {
() => {
// Module: crate::path_utils
// Provides: {"borrow_of_local_data"}
// Dependencies: {}
# [doc = " Determines if a given borrow is borrowing local data"] # [doc = " This is called for all Yield expressions on movable coroutines"] pub (super) fn borrow_of_local_data (place : Place < '_ >) -> bool { ! place . is_indirect () }
};
}
