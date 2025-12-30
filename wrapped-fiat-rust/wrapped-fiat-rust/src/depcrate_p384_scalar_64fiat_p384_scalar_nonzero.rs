// Generated macro for fiat_p384_scalar_nonzero (function)
macro_rules! Depcrate_p384_scalar_64fiat_p384_scalar_nonzero {
() => {
// Module: crate::p384_scalar_64
// Provides: {"fiat_p384_scalar_nonzero"}
// Dependencies: {}
# [doc = " The function fiat_p384_scalar_nonzero outputs a single non-zero word if the input is non-zero and zero otherwise."] # [doc = ""] # [doc = " Preconditions:"] # [doc = "   0 ≤ eval arg1 < m"] # [doc = " Postconditions:"] # [doc = "   out1 = 0 ↔ eval (from_montgomery arg1) mod m = 0"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0xffffffffffffffff]"] # [inline] pub const fn fiat_p384_scalar_nonzero (out1 : & mut u64 , arg1 : & [u64 ; 6]) { let x1 : u64 = ((* IndexConst (arg1) . index (0)) | ((* IndexConst (arg1) . index (1)) | ((* IndexConst (arg1) . index (2)) | ((* IndexConst (arg1) . index (3)) | ((* IndexConst (arg1) . index (4)) | (* IndexConst (arg1) . index (5))))))) ; * out1 = x1 ; }
};
}
