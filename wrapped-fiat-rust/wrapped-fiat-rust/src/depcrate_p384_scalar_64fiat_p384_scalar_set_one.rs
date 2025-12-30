// Generated macro for fiat_p384_scalar_set_one (function)
macro_rules! Depcrate_p384_scalar_64fiat_p384_scalar_set_one {
() => {
// Module: crate::p384_scalar_64
// Provides: {"fiat_p384_scalar_set_one"}
// Dependencies: {}
# [doc = " The function fiat_p384_scalar_set_one returns the field element one in the Montgomery domain."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) mod m = 1 mod m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [inline] pub const fn fiat_p384_scalar_set_one (mut out1 : & mut fiat_p384_scalar_montgomery_domain_field_element) { * IndexConst (& mut out1) . index_mut (0) = 0x1313e695333ad68d ; * IndexConst (& mut out1) . index_mut (1) = 0xa7e5f24db74f5885 ; * IndexConst (& mut out1) . index_mut (2) = 0x389cb27e0bc8d220 ; * IndexConst (& mut out1) . index_mut (3) = (0x0 as u64) ; * IndexConst (& mut out1) . index_mut (4) = (0x0 as u64) ; * IndexConst (& mut out1) . index_mut (5) = (0x0 as u64) ; }
};
}
