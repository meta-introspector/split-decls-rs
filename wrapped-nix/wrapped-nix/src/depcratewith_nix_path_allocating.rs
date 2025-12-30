// Generated macro for with_nix_path_allocating (function)
macro_rules! Depcratewith_nix_path_allocating {
() => {
// Module: crate
// Provides: {"with_nix_path_allocating"}
// Dependencies: {}
# [cold] # [inline (never)] fn with_nix_path_allocating < T , F > (from : & [u8] , f : F) -> Result < T > where F : FnOnce (& CStr) -> T , { match CString :: new (from) { Ok (s) => Ok (f (& s)) , Err (_) => Err (Errno :: EINVAL) , } }
};
}
