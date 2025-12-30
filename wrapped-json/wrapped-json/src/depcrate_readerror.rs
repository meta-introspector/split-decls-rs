// Generated macro for error (function)
macro_rules! Depcrate_readerror {
() => {
// Module: crate::read
// Provides: {"error"}
// Dependencies: {}
fn error < 'de , R , T > (read : & R , reason : ErrorCode) -> Result < T > where R : ? Sized + Read < 'de > , { let position = read . position () ; Err (Error :: syntax (reason , position . line , position . column)) }
};
}
