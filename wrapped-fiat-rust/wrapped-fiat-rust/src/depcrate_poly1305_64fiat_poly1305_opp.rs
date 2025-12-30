// Generated macro for fiat_poly1305_opp (function)
macro_rules! Depcrate_poly1305_64fiat_poly1305_opp {
() => {
// Module: crate::poly1305_64
// Provides: {"fiat_poly1305_opp"}
// Dependencies: {}
# [doc = " The function fiat_poly1305_opp negates a field element."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval out1 mod m = -eval arg1 mod m"] # [doc = ""] # [inline] pub const fn fiat_poly1305_opp (mut out1 : & mut fiat_poly1305_loose_field_element , arg1 : & fiat_poly1305_tight_field_element) { let x1 : u64 = (0x1ffffffffff6 - (* IndexConst (arg1) . index (0))) ; let x2 : u64 = (0xffffffffffe - (* IndexConst (arg1) . index (1))) ; let x3 : u64 = (0xffffffffffe - (* IndexConst (arg1) . index (2))) ; * IndexConst (& mut out1) . index_mut (0) = x1 ; * IndexConst (& mut out1) . index_mut (1) = x2 ; * IndexConst (& mut out1) . index_mut (2) = x3 ; }
};
}
