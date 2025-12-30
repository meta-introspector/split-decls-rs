// Generated macro for fiat_sm2_scalar_msat (function)
macro_rules! Depcrate_sm2_scalar_32fiat_sm2_scalar_msat {
() => {
// Module: crate::sm2_scalar_32
// Provides: {"fiat_sm2_scalar_msat"}
// Dependencies: {}
# [doc = " The function fiat_sm2_scalar_msat returns the saturated representation of the prime modulus."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   twos_complement_eval out1 = m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]]"] # [inline] pub const fn fiat_sm2_scalar_msat (mut out1 : & mut [u32 ; 9]) { * IndexConst (& mut out1) . index_mut (0) = 0x39d54123 ; * IndexConst (& mut out1) . index_mut (1) = 0x53bbf409 ; * IndexConst (& mut out1) . index_mut (2) = 0x21c6052b ; * IndexConst (& mut out1) . index_mut (3) = 0x7203df6b ; * IndexConst (& mut out1) . index_mut (4) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (5) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (6) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (7) = 0xfffffffe ; * IndexConst (& mut out1) . index_mut (8) = (0x0 as u32) ; }
};
}
