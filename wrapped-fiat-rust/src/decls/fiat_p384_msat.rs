macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_p384_msat {
    () => {
        deps!();
        # [doc = " The function fiat_p384_msat returns the saturated representation of the prime modulus."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   twos_complement_eval out1 = m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [inline] pub const fn fiat_p384_msat (mut out1 : & mut [u64 ; 7]) { * IndexConst (& mut out1) . index_mut (0) = 0xffffffff ; * IndexConst (& mut out1) . index_mut (1) = 0xffffffff00000000 ; * IndexConst (& mut out1) . index_mut (2) = 0xfffffffffffffffe ; * IndexConst (& mut out1) . index_mut (3) = 0xffffffffffffffff ; * IndexConst (& mut out1) . index_mut (4) = 0xffffffffffffffff ; * IndexConst (& mut out1) . index_mut (5) = 0xffffffffffffffff ; * IndexConst (& mut out1) . index_mut (6) = (0x0 as u64) ; }
    };
}

fiat_p384_msat!()