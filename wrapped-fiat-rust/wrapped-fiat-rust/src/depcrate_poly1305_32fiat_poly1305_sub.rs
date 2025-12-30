// Generated macro for fiat_poly1305_sub (function)
macro_rules! Depcrate_poly1305_32fiat_poly1305_sub {
() => {
// Module: crate::poly1305_32
// Provides: {"fiat_poly1305_sub"}
// Dependencies: {}
# [doc = " The function fiat_poly1305_sub subtracts two field elements."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval out1 mod m = (eval arg1 - eval arg2) mod m"] # [doc = ""] # [inline] pub const fn fiat_poly1305_sub (mut out1 : & mut fiat_poly1305_loose_field_element , arg1 : & fiat_poly1305_tight_field_element , arg2 : & fiat_poly1305_tight_field_element) { let x1 : u32 = ((0x7fffff6 + (* IndexConst (arg1) . index (0))) - (* IndexConst (arg2) . index (0))) ; let x2 : u32 = ((0x7fffffe + (* IndexConst (arg1) . index (1))) - (* IndexConst (arg2) . index (1))) ; let x3 : u32 = ((0x7fffffe + (* IndexConst (arg1) . index (2))) - (* IndexConst (arg2) . index (2))) ; let x4 : u32 = ((0x7fffffe + (* IndexConst (arg1) . index (3))) - (* IndexConst (arg2) . index (3))) ; let x5 : u32 = ((0x7fffffe + (* IndexConst (arg1) . index (4))) - (* IndexConst (arg2) . index (4))) ; * IndexConst (& mut out1) . index_mut (0) = x1 ; * IndexConst (& mut out1) . index_mut (1) = x2 ; * IndexConst (& mut out1) . index_mut (2) = x3 ; * IndexConst (& mut out1) . index_mut (3) = x4 ; * IndexConst (& mut out1) . index_mut (4) = x5 ; }
};
}
