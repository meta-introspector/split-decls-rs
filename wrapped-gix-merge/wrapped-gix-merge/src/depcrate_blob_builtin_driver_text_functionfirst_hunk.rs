// Generated macro for first_hunk (function)
macro_rules! Depcrate_blob_builtin_driver_text_functionfirst_hunk {
() => {
// Module: crate::blob::builtin_driver::text::function
// Provides: {"first_hunk"}
// Dependencies: {}
fn first_hunk < 'a > (front : & 'a [Hunk] , ours : & 'a [Hunk] , theirs : & 'a [Hunk] , back : & 'a [Hunk]) -> & 'a Hunk { front . first () . or (ours . first ()) . or (theirs . first ()) . or (back . first ()) . expect ("at least one hunk - we aborted if there are none anywhere") }
};
}
