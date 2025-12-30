// Generated macro for impl_1941 (impl)
macro_rules! Depcrate_isa_riscv64_abiimpl_1941 {
() => {
// Module: crate::isa::riscv64::abi
// Provides: {"impl_1941"}
// Dependencies: {}
impl RiscvFlags { pub (crate) fn min_vec_reg_size (& self) -> u64 { let entries = [(self . has_zvl65536b () , 65536) , (self . has_zvl32768b () , 32768) , (self . has_zvl16384b () , 16384) , (self . has_zvl8192b () , 8192) , (self . has_zvl4096b () , 4096) , (self . has_zvl2048b () , 2048) , (self . has_zvl1024b () , 1024) , (self . has_zvl512b () , 512) , (self . has_zvl256b () , 256) , (self . has_v () , 128) , (self . has_zvl128b () , 128) , (self . has_zvl64b () , 64) , (self . has_zvl32b () , 32) ,] ; for (has_flag , size) in entries . into_iter () { if ! has_flag { continue ; } return std :: cmp :: min (size , 1024) ; } return 0 ; } }
};
}
