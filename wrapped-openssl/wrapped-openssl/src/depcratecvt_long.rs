// Generated macro for cvt_long (function)
macro_rules! Depcratecvt_long {
() => {
// Module: crate
// Provides: {"cvt_long"}
// Dependencies: {}
# [inline] # [cfg (ossl300)] fn cvt_long (r : c_long) -> Result < c_long , ErrorStack > { if r <= 0 { Err (ErrorStack :: get ()) } else { Ok (r) } }
};
}
