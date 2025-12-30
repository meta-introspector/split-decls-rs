// Generated macro for fiat_poly1305_relax (function)
macro_rules! Depcrate_poly1305_32fiat_poly1305_relax {
() => {
// Module: crate::poly1305_32
// Provides: {"fiat_poly1305_relax"}
// Dependencies: {}
# [doc = " The function fiat_poly1305_relax is the identity function converting from tight field elements to loose field elements."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = arg1"] # [doc = ""] # [inline] pub const fn fiat_poly1305_relax (mut out1 : & mut fiat_poly1305_loose_field_element , arg1 : & fiat_poly1305_tight_field_element) { let x1 : u32 = (* IndexConst (arg1) . index (0)) ; let x2 : u32 = (* IndexConst (arg1) . index (1)) ; let x3 : u32 = (* IndexConst (arg1) . index (2)) ; let x4 : u32 = (* IndexConst (arg1) . index (3)) ; let x5 : u32 = (* IndexConst (arg1) . index (4)) ; * IndexConst (& mut out1) . index_mut (0) = x1 ; * IndexConst (& mut out1) . index_mut (1) = x2 ; * IndexConst (& mut out1) . index_mut (2) = x3 ; * IndexConst (& mut out1) . index_mut (3) = x4 ; * IndexConst (& mut out1) . index_mut (4) = x5 ; }
};
}
