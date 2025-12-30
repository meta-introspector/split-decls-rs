// Generated macro for Progress (type)
macro_rules! Depcrate_buildProgress {
() => {
// Module: crate::build
// Provides: {"Progress"}
// Dependencies: {}
# [doc = " Checkout progress notification callback."] # [doc = ""] # [doc = " The first argument is the path for the notification, the next is the number"] # [doc = " of completed steps so far, and the final is the total number of steps."] pub type Progress < 'a > = dyn FnMut (Option < & Path > , usize , usize) + 'a ;
};
}
