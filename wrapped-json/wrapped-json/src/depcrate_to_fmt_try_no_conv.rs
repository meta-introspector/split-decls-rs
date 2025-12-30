// Generated macro for _try_no_conv (macro)
macro_rules! Depcrate_to_fmt_try_no_conv {
() => {
// Module: crate::to_fmt
// Provides: {"_try_no_conv"}
// Dependencies: {}
macro_rules ! _try_no_conv { ($ e : expr) => { match ($ e) { Ok (()) => () , Err (e) => return Err (e) , } } ; }
};
}
