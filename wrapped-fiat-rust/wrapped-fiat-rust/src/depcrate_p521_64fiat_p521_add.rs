// Generated macro for fiat_p521_add (function)
macro_rules! Depcrate_p521_64fiat_p521_add {
() => {
// Module: crate::p521_64
// Provides: {"fiat_p521_add"}
// Dependencies: {}
# [doc = " The function fiat_p521_add adds two field elements."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval out1 mod m = (eval arg1 + eval arg2) mod m"] # [doc = ""] # [inline] pub const fn fiat_p521_add (mut out1 : & mut fiat_p521_loose_field_element , arg1 : & fiat_p521_tight_field_element , arg2 : & fiat_p521_tight_field_element) { let x1 : u64 = ((* IndexConst (arg1) . index (0)) + (* IndexConst (arg2) . index (0))) ; let x2 : u64 = ((* IndexConst (arg1) . index (1)) + (* IndexConst (arg2) . index (1))) ; let x3 : u64 = ((* IndexConst (arg1) . index (2)) + (* IndexConst (arg2) . index (2))) ; let x4 : u64 = ((* IndexConst (arg1) . index (3)) + (* IndexConst (arg2) . index (3))) ; let x5 : u64 = ((* IndexConst (arg1) . index (4)) + (* IndexConst (arg2) . index (4))) ; let x6 : u64 = ((* IndexConst (arg1) . index (5)) + (* IndexConst (arg2) . index (5))) ; let x7 : u64 = ((* IndexConst (arg1) . index (6)) + (* IndexConst (arg2) . index (6))) ; let x8 : u64 = ((* IndexConst (arg1) . index (7)) + (* IndexConst (arg2) . index (7))) ; let x9 : u64 = ((* IndexConst (arg1) . index (8)) + (* IndexConst (arg2) . index (8))) ; * IndexConst (& mut out1) . index_mut (0) = x1 ; * IndexConst (& mut out1) . index_mut (1) = x2 ; * IndexConst (& mut out1) . index_mut (2) = x3 ; * IndexConst (& mut out1) . index_mut (3) = x4 ; * IndexConst (& mut out1) . index_mut (4) = x5 ; * IndexConst (& mut out1) . index_mut (5) = x6 ; * IndexConst (& mut out1) . index_mut (6) = x7 ; * IndexConst (& mut out1) . index_mut (7) = x8 ; * IndexConst (& mut out1) . index_mut (8) = x9 ; }
};
}
