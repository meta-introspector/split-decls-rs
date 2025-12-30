// Generated macro for tests (module)
macro_rules! Depcrate_block_apitests {
() => {
// Module: crate::block_api
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { belt_compress , read_u32s } ; use hex_literal :: hex ; # [doc = " Test vectors for the `belt-compress` functions from the"] # [doc = " specification (Table A.8)."] # [test] fn compress () { let x = & hex ! ("B194BAC8 0A08F53B 366D008E 584A5DE4" "8504FA9D 1BB6C7AC 252E72C2 02FDCE0D" "5BE3D612 17B96181 FE6786AD 716B890B" "5CB0C0FF 33C356B8 35C405AE D8E07F99") ; let expected_s = & hex ! ("46FE7425 C9B181EB 41DFEE3E 72163D5A") ; let expected_y = & hex ! ("ED2F5481 D593F40D 87FCE37D 6BC1A2E1" "B7D1A2CC 975C82D3 C0497488 C90D99D8") ; let x1 = read_u32s (& x [.. 16]) ; let x2 = read_u32s (& x [16 .. 32]) ; let x34 = read_u32s (& x [32 ..]) ; let (s , y) = belt_compress (x1 , x2 , x34) ; assert_eq ! (s , read_u32s (expected_s)) ; assert_eq ! (y , read_u32s (expected_y)) ; } }
};
}
