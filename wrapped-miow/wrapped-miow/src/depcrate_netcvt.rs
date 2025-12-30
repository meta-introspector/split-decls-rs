// Generated macro for cvt (function)
macro_rules! Depcrate_netcvt {
() => {
// Module: crate::net
// Provides: {"cvt"}
// Dependencies: {}
fn cvt (i : i32 , size : u32) -> io :: Result < Option < usize > > { if i == SOCKET_ERROR { last_err () } else { Ok (Some (size as usize)) } }
};
}
