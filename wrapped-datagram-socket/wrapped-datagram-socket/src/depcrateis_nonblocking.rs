// Generated macro for is_nonblocking (function)
macro_rules! Depcrateis_nonblocking {
() => {
// Module: crate
// Provides: {"is_nonblocking"}
// Dependencies: {}
# [cfg (all (unix , not (target_os = "linux")))] pub fn is_nonblocking (_fd : & impl AsRawFd) -> std :: io :: Result < bool > { Ok (true) }
};
}
