// Generated macro for win32_error (function)
macro_rules! Depcratewin32_error {
() => {
// Module: crate
// Provides: {"win32_error"}
// Dependencies: {}
fn win32_error (result : u32) -> Result < () > { if result == 0 { Ok (()) } else { Err (Error :: from_hresult (WIN32_ERROR (result) . to_hresult ())) } }
};
}
