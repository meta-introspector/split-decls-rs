// Generated macro for safe (function)
macro_rules! Depcrate_testersafe {
() => {
// Module: crate::tester
// Provides: {"safe"}
// Dependencies: {}
fn safe < T , F > (fun : F) -> Result < T , String > where F : FnOnce () -> T , F : 'static , T : 'static , { panic :: catch_unwind (panic :: AssertUnwindSafe (fun)) . map_err (| any_err | { if let Some (& s) = any_err . downcast_ref :: < & str > () { s . to_owned () } else if let Some (s) = any_err . downcast_ref :: < String > () { s . to_owned () } else { "UNABLE TO SHOW RESULT OF PANIC." . to_owned () } }) }
};
}
