// Generated macro for fiat_p384_scalar_msat (function)
macro_rules! Depcrate_p384_scalar_32fiat_p384_scalar_msat {
() => {
// Module: crate::p384_scalar_32
// Provides: {"fiat_p384_scalar_msat"}
// Dependencies: {}
# [doc = " The function fiat_p384_scalar_msat returns the saturated representation of the prime modulus."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   twos_complement_eval out1 = m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]]"] # [inline] pub const fn fiat_p384_scalar_msat (mut out1 : & mut [u32 ; 13]) { * IndexConst (& mut out1) . index_mut (0) = 0xccc52973 ; * IndexConst (& mut out1) . index_mut (1) = 0xecec196a ; * IndexConst (& mut out1) . index_mut (2) = 0x48b0a77a ; * IndexConst (& mut out1) . index_mut (3) = 0x581a0db2 ; * IndexConst (& mut out1) . index_mut (4) = 0xf4372ddf ; * IndexConst (& mut out1) . index_mut (5) = 0xc7634d81 ; * IndexConst (& mut out1) . index_mut (6) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (7) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (8) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (9) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (10) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (11) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (12) = (0x0 as u32) ; }
};
}
