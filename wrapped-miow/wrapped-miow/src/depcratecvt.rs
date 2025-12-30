// Generated macro for cvt (function)
macro_rules! Depcratecvt {
() => {
// Module: crate
// Provides: {"cvt"}
// Dependencies: {}
fn cvt (i : BOOL) -> io :: Result < BOOL > { if i == 0 { Err (io :: Error :: last_os_error ()) } else { Ok (i) } }
};
}
