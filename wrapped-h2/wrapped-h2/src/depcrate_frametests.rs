// Generated macro for tests (module)
macro_rules! Depcrate_frametests {
() => {
// Module: crate::frame
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn test_unpack_octets_4 () { let buf : [u8 ; 4] = [0 , 0 , 0 , 1] ; assert_eq ! (1u32 , unpack_octets_4 ! (buf , 0 , u32)) ; } }
};
}
