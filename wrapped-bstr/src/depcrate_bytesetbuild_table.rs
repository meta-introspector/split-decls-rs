// Generated macro for build_table (function)
macro_rules! Depcrate_bytesetbuild_table {
() => {
// Module: crate::byteset
// Provides: {"build_table"}
// Dependencies: {}
# [inline] fn build_table (byteset : & [u8]) -> [u8 ; 256] { let mut table = [0u8 ; 256] ; for & b in byteset { table [b as usize] = 1 ; } table }
};
}
