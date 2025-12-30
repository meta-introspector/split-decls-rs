// Generated macro for fiat_p448_opp (function)
macro_rules! Depcrate_p448_solinas_64fiat_p448_opp {
() => {
// Module: crate::p448_solinas_64
// Provides: {"fiat_p448_opp"}
// Dependencies: {}
# [doc = " The function fiat_p448_opp negates a field element."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval out1 mod m = -eval arg1 mod m"] # [doc = ""] # [inline] pub const fn fiat_p448_opp (mut out1 : & mut fiat_p448_loose_field_element , arg1 : & fiat_p448_tight_field_element) { let x1 : u64 = (0x1fffffffffffffe - (* IndexConst (arg1) . index (0))) ; let x2 : u64 = (0x1fffffffffffffe - (* IndexConst (arg1) . index (1))) ; let x3 : u64 = (0x1fffffffffffffe - (* IndexConst (arg1) . index (2))) ; let x4 : u64 = (0x1fffffffffffffe - (* IndexConst (arg1) . index (3))) ; let x5 : u64 = (0x1fffffffffffffc - (* IndexConst (arg1) . index (4))) ; let x6 : u64 = (0x1fffffffffffffe - (* IndexConst (arg1) . index (5))) ; let x7 : u64 = (0x1fffffffffffffe - (* IndexConst (arg1) . index (6))) ; let x8 : u64 = (0x1fffffffffffffe - (* IndexConst (arg1) . index (7))) ; * IndexConst (& mut out1) . index_mut (0) = x1 ; * IndexConst (& mut out1) . index_mut (1) = x2 ; * IndexConst (& mut out1) . index_mut (2) = x3 ; * IndexConst (& mut out1) . index_mut (3) = x4 ; * IndexConst (& mut out1) . index_mut (4) = x5 ; * IndexConst (& mut out1) . index_mut (5) = x6 ; * IndexConst (& mut out1) . index_mut (6) = x7 ; * IndexConst (& mut out1) . index_mut (7) = x8 ; }
};
}
