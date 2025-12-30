// Generated macro for cvt (function)
macro_rules! Depcratecvt {
() => {
// Module: crate
// Provides: {"cvt"}
// Dependencies: {}
# [inline] fn cvt (r : std :: os :: raw :: c_int) -> Result < std :: os :: raw :: c_int , openssl :: error :: ErrorStack > { if r <= 0 { Err (openssl :: error :: ErrorStack :: get ()) } else { Ok (r) } }
};
}
