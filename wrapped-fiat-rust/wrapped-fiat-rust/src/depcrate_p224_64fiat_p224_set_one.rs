// Generated macro for fiat_p224_set_one (function)
macro_rules! Depcrate_p224_64fiat_p224_set_one {
() => {
// Module: crate::p224_64
// Provides: {"fiat_p224_set_one"}
// Dependencies: {}
# [doc = " The function fiat_p224_set_one returns the field element one in the Montgomery domain."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) mod m = 1 mod m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [inline] pub const fn fiat_p224_set_one (mut out1 : & mut fiat_p224_montgomery_domain_field_element) { * IndexConst (& mut out1) . index_mut (0) = 0xffffffff00000000 ; * IndexConst (& mut out1) . index_mut (1) = 0xffffffffffffffff ; * IndexConst (& mut out1) . index_mut (2) = (0x0 as u64) ; * IndexConst (& mut out1) . index_mut (3) = (0x0 as u64) ; }
};
}
