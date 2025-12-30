// Generated macro for cvt_p (function)
macro_rules! Depcratecvt_p {
() => {
// Module: crate
// Provides: {"cvt_p"}
// Dependencies: {}
# [inline] fn cvt_p < T > (r : * mut T) -> Result < * mut T , openssl :: error :: ErrorStack > { if r . is_null () { Err (openssl :: error :: ErrorStack :: get ()) } else { Ok (r) } }
};
}
