macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_secp256k1_montgomery_msat {
    () => {
        deps!();
        # [doc = " The function fiat_secp256k1_montgomery_msat returns the saturated representation of the prime modulus."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   twos_complement_eval out1 = m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [inline] pub const fn fiat_secp256k1_montgomery_msat (mut out1 : & mut [u64 ; 5]) { * IndexConst (& mut out1) . index_mut (0) = 0xfffffffefffffc2f ; * IndexConst (& mut out1) . index_mut (1) = 0xffffffffffffffff ; * IndexConst (& mut out1) . index_mut (2) = 0xffffffffffffffff ; * IndexConst (& mut out1) . index_mut (3) = 0xffffffffffffffff ; * IndexConst (& mut out1) . index_mut (4) = (0x0 as u64) ; }
    };
}

fiat_secp256k1_montgomery_msat!()