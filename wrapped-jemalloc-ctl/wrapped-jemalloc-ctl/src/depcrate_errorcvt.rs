// Generated macro for cvt (function)
macro_rules! Depcrate_errorcvt {
() => {
// Module: crate::error
// Provides: {"cvt"}
// Dependencies: {}
pub (crate) fn cvt (ret : c_int) -> Result < () > { match ret { 0 => Ok (()) , v => Err (Error (unsafe { NonZeroCInt :: new_unchecked (v as _) })) , } }
};
}
