// Generated macro for AUTH_TYPE_FIX_ (macro)
macro_rules! DepcrateAUTH_TYPE_FIX_ {
() => {
// Module: crate
// Provides: {"AUTH_TYPE_FIX_"}
// Dependencies: {}
# [cfg (feature = "SecKeychain")] # [allow (non_snake_case)] macro_rules ! AUTH_TYPE_FIX_ { ($ code : expr) => { $ crate :: FourCharCode :: from_be ($ code) } ; }
};
}
