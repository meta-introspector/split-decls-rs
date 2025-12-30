// Generated macro for fiat_p384_set_one (function)
macro_rules! Depcrate_p384_32fiat_p384_set_one {
() => {
// Module: crate::p384_32
// Provides: {"fiat_p384_set_one"}
// Dependencies: {}
# [doc = " The function fiat_p384_set_one returns the field element one in the Montgomery domain."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) mod m = 1 mod m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [inline] pub const fn fiat_p384_set_one (mut out1 : & mut fiat_p384_montgomery_domain_field_element) { * IndexConst (& mut out1) . index_mut (0) = (0x1 as u32) ; * IndexConst (& mut out1) . index_mut (1) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (2) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (3) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (4) = (0x1 as u32) ; * IndexConst (& mut out1) . index_mut (5) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (6) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (7) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (8) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (9) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (10) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (11) = (0x0 as u32) ; }
};
}
