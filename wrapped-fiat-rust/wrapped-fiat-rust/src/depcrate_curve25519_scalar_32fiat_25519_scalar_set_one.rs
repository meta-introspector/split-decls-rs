// Generated macro for fiat_25519_scalar_set_one (function)
macro_rules! Depcrate_curve25519_scalar_32fiat_25519_scalar_set_one {
() => {
// Module: crate::curve25519_scalar_32
// Provides: {"fiat_25519_scalar_set_one"}
// Dependencies: {}
# [doc = " The function fiat_25519_scalar_set_one returns the field element one in the Montgomery domain."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) mod m = 1 mod m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [inline] pub const fn fiat_25519_scalar_set_one (mut out1 : & mut fiat_25519_scalar_montgomery_domain_field_element) { * IndexConst (& mut out1) . index_mut (0) = 0x8d98951d ; * IndexConst (& mut out1) . index_mut (1) = 0xd6ec3174 ; * IndexConst (& mut out1) . index_mut (2) = 0x737dcf70 ; * IndexConst (& mut out1) . index_mut (3) = 0xc6ef5bf4 ; * IndexConst (& mut out1) . index_mut (4) = 0xfffffffe ; * IndexConst (& mut out1) . index_mut (5) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (6) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (7) = 0xfffffff ; }
};
}
