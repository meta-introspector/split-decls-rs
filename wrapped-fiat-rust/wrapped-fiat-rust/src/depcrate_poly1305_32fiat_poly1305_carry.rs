// Generated macro for fiat_poly1305_carry (function)
macro_rules! Depcrate_poly1305_32fiat_poly1305_carry {
() => {
// Module: crate::poly1305_32
// Provides: {"fiat_poly1305_carry"}
// Dependencies: {}
# [doc = " The function fiat_poly1305_carry reduces a field element."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval out1 mod m = eval arg1 mod m"] # [doc = ""] # [inline] pub const fn fiat_poly1305_carry (mut out1 : & mut fiat_poly1305_tight_field_element , arg1 : & fiat_poly1305_loose_field_element) { let x1 : u32 = (* IndexConst (arg1) . index (0)) ; let x2 : u32 = ((x1 >> 26) + (* IndexConst (arg1) . index (1))) ; let x3 : u32 = ((x2 >> 26) + (* IndexConst (arg1) . index (2))) ; let x4 : u32 = ((x3 >> 26) + (* IndexConst (arg1) . index (3))) ; let x5 : u32 = ((x4 >> 26) + (* IndexConst (arg1) . index (4))) ; let x6 : u32 = ((x1 & 0x3ffffff) + ((x5 >> 26) * 0x5)) ; let x7 : u32 = ((((x6 >> 26) as fiat_poly1305_u1) as u32) + (x2 & 0x3ffffff)) ; let x8 : u32 = (x6 & 0x3ffffff) ; let x9 : u32 = (x7 & 0x3ffffff) ; let x10 : u32 = ((((x7 >> 26) as fiat_poly1305_u1) as u32) + (x3 & 0x3ffffff)) ; let x11 : u32 = (x4 & 0x3ffffff) ; let x12 : u32 = (x5 & 0x3ffffff) ; * IndexConst (& mut out1) . index_mut (0) = x8 ; * IndexConst (& mut out1) . index_mut (1) = x9 ; * IndexConst (& mut out1) . index_mut (2) = x10 ; * IndexConst (& mut out1) . index_mut (3) = x11 ; * IndexConst (& mut out1) . index_mut (4) = x12 ; }
};
}
