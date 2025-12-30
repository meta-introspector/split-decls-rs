// Generated macro for panicked (function)
macro_rules! Depcrate_panicpanicked {
() => {
// Module: crate::panic
// Provides: {"panicked"}
// Dependencies: {}
pub fn panicked () -> bool { LAST_ERROR . with (| slot | slot . borrow () . is_some ()) }
};
}
