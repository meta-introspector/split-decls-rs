// Generated macro for initialize (function)
macro_rules! Depcrate_cominitialize {
() => {
// Module: crate::com
// Provides: {"initialize"}
// Dependencies: {}
pub fn initialize () -> Result < () , HRESULT > { let err = unsafe { CoInitializeEx (null () , COINIT_MULTITHREADED . try_into () . unwrap ()) } ; if err != S_OK && err != S_FALSE { Err (err) } else { Ok (()) } }
};
}
