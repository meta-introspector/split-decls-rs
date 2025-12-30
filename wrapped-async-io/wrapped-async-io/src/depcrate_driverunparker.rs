// Generated macro for unparker (function)
macro_rules! Depcrate_driverunparker {
() => {
// Module: crate::driver
// Provides: {"unparker"}
// Dependencies: {}
# [doc = " Unparker for the \"async-io\" thread."] fn unparker () -> & 'static parking :: Unparker { static UNPARKER : OnceLock < parking :: Unparker > = OnceLock :: new () ; UNPARKER . get_or_init (| | { let (parker , unparker) = parking :: pair () ; thread :: Builder :: new () . name ("async-io" . to_string ()) . spawn (move | | main_loop (parker)) . expect ("cannot spawn async-io thread") ; unparker }) }
};
}
