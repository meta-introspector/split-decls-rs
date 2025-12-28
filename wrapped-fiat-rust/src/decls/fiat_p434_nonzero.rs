macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_p434_nonzero {
    () => {
        deps!();
        # [doc = " The function fiat_p434_nonzero outputs a single non-zero word if the input is non-zero and zero otherwise."] # [doc = ""] # [doc = " Preconditions:"] # [doc = "   0 ≤ eval arg1 < m"] # [doc = " Postconditions:"] # [doc = "   out1 = 0 ↔ eval (from_montgomery arg1) mod m = 0"] # [doc = ""] # [doc = " Input Bounds:"] # [doc = "   arg1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [doc = " Output Bounds:"] # [doc = "   out1: [0x0 ~> 0xffffffffffffffff]"] # [inline] pub const fn fiat_p434_nonzero (out1 : & mut u64 , arg1 : & [u64 ; 7]) { let x1 : u64 = ((* IndexConst (arg1) . index (0)) | ((* IndexConst (arg1) . index (1)) | ((* IndexConst (arg1) . index (2)) | ((* IndexConst (arg1) . index (3)) | ((* IndexConst (arg1) . index (4)) | ((* IndexConst (arg1) . index (5)) | (* IndexConst (arg1) . index (6)))))))) ; * out1 = x1 ; }
    };
}

fiat_p434_nonzero!()