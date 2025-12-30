// Generated macro for fiat_sm2_msat (function)
macro_rules! Depcrate_sm2_32fiat_sm2_msat {
() => {
// Module: crate::sm2_32
// Provides: {"fiat_sm2_msat"}
// Dependencies: {}
# [doc = " The function fiat_sm2_msat returns the saturated representation of the prime modulus."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   twos_complement_eval out1 = m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]]"] # [inline] pub const fn fiat_sm2_msat (mut out1 : & mut [u32 ; 9]) { * IndexConst (& mut out1) . index_mut (0) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (1) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (2) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (3) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (4) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (5) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (6) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (7) = 0xfffffffe ; * IndexConst (& mut out1) . index_mut (8) = (0x0 as u32) ; }
};
}
