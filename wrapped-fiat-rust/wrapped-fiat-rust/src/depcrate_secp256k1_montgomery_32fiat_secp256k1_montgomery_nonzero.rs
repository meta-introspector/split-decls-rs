// Generated macro for fiat_secp256k1_montgomery_nonzero (function)
macro_rules! Depcrate_secp256k1_montgomery_32fiat_secp256k1_montgomery_nonzero {
() => {
// Module: crate::secp256k1_montgomery_32
// Provides: {"fiat_secp256k1_montgomery_nonzero"}
// Dependencies: {}
# [doc = " The function fiat_secp256k1_montgomery_nonzero outputs a single non-zero word if the input is non-zero and zero otherwise."] # [doc = ""] # [doc = " Preconditions:"] # [doc = "   0 ≤ eval arg1 < m"] # [doc = " Postconditions:"] # [doc = "   out1 = 0 ↔ eval (from_montgomery arg1) mod m = 0"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0xffffffff]"] # [inline] pub const fn fiat_secp256k1_montgomery_nonzero (out1 : & mut u32 , arg1 : & [u32 ; 8]) { let x1 : u32 = ((* IndexConst (arg1) . index (0)) | ((* IndexConst (arg1) . index (1)) | ((* IndexConst (arg1) . index (2)) | ((* IndexConst (arg1) . index (3)) | ((* IndexConst (arg1) . index (4)) | ((* IndexConst (arg1) . index (5)) | ((* IndexConst (arg1) . index (6)) | (* IndexConst (arg1) . index (7))))))))) ; * out1 = x1 ; }
};
}
