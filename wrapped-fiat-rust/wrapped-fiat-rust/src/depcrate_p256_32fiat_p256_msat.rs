// Generated macro for fiat_p256_msat (function)
macro_rules! Depcrate_p256_32fiat_p256_msat {
() => {
// Module: crate::p256_32
// Provides: {"fiat_p256_msat"}
// Dependencies: {}
# [doc = " The function fiat_p256_msat returns the saturated representation of the prime modulus."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   twos_complement_eval out1 = m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]]"] # [inline] pub const fn fiat_p256_msat (mut out1 : & mut [u32 ; 9]) { * IndexConst (& mut out1) . index_mut (0) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (1) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (2) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (3) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (4) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (5) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (6) = (0x1 as u32) ; * IndexConst (& mut out1) . index_mut (7) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (8) = (0x0 as u32) ; }
};
}
