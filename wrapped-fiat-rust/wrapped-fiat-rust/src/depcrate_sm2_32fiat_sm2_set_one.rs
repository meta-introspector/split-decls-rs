// Generated macro for fiat_sm2_set_one (function)
macro_rules! Depcrate_sm2_32fiat_sm2_set_one {
() => {
// Module: crate::sm2_32
// Provides: {"fiat_sm2_set_one"}
// Dependencies: {}
# [doc = " The function fiat_sm2_set_one returns the field element one in the Montgomery domain."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) mod m = 1 mod m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [inline] pub const fn fiat_sm2_set_one (mut out1 : & mut fiat_sm2_montgomery_domain_field_element) { * IndexConst (& mut out1) . index_mut (0) = (0x1 as u32) ; * IndexConst (& mut out1) . index_mut (1) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (2) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (3) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (4) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (5) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (6) = (0x0 as u32) ; * IndexConst (& mut out1) . index_mut (7) = (0x1 as u32) ; }
};
}
