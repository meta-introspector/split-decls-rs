// Generated macro for check (function)
macro_rules! Depcrate_paniccheck {
() => {
// Module: crate::panic
// Provides: {"check"}
// Dependencies: {}
pub fn check () { let err = LAST_ERROR . with (| slot | slot . borrow_mut () . take ()) ; if let Some (err) = err { std :: panic :: resume_unwind (err) ; } }
};
}
