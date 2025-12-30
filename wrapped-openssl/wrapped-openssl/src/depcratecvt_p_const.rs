// Generated macro for cvt_p_const (function)
macro_rules! Depcratecvt_p_const {
() => {
// Module: crate
// Provides: {"cvt_p_const"}
// Dependencies: {}
# [inline] fn cvt_p_const < T > (r : * const T) -> Result < * const T , ErrorStack > { if r . is_null () { Err (ErrorStack :: get ()) } else { Ok (r) } }
};
}
