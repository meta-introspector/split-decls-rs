macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_p434_msat {
    () => {
        deps!();
        # [doc = " The function fiat_p434_msat returns the saturated representation of the prime modulus."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   twos_complement_eval out1 = m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [inline] pub const fn fiat_p434_msat (mut out1 : & mut [u64 ; 8]) { * IndexConst (& mut out1) . index_mut (0) = 0xffffffffffffffff ; * IndexConst (& mut out1) . index_mut (1) = 0xffffffffffffffff ; * IndexConst (& mut out1) . index_mut (2) = 0xffffffffffffffff ; * IndexConst (& mut out1) . index_mut (3) = 0xfdc1767ae2ffffff ; * IndexConst (& mut out1) . index_mut (4) = 0x7bc65c783158aea3 ; * IndexConst (& mut out1) . index_mut (5) = 0x6cfc5fd681c52056 ; * IndexConst (& mut out1) . index_mut (6) = 0x2341f27177344 ; * IndexConst (& mut out1) . index_mut (7) = (0x0 as u64) ; }
    };
}

fiat_p434_msat!()