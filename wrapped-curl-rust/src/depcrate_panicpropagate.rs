// Generated macro for propagate (function)
macro_rules! Depcrate_panicpropagate {
() => {
// Module: crate::panic
// Provides: {"propagate"}
// Dependencies: {}
pub fn propagate () { if let Ok (Some (t)) = LAST_ERROR . try_with (| slot | slot . borrow_mut () . take ()) { panic :: resume_unwind (t) } }
};
}
