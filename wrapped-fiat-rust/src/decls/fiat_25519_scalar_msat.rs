macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_25519_scalar_msat {
    () => {
        deps!();
        # [doc = " The function fiat_25519_scalar_msat returns the saturated representation of the prime modulus."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   twos_complement_eval out1 = m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [doc = " Output Bounds:"] # [doc = "   out1: [[0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff], [0x0 ~> 0xffffffffffffffff]]"] # [inline] pub const fn fiat_25519_scalar_msat (mut out1 : & mut [u64 ; 5]) { * IndexConst (& mut out1) . index_mut (0) = 0x5812631a5cf5d3ed ; * IndexConst (& mut out1) . index_mut (1) = 0x14def9dea2f79cd6 ; * IndexConst (& mut out1) . index_mut (2) = (0x0 as u64) ; * IndexConst (& mut out1) . index_mut (3) = 0x1000000000000000 ; * IndexConst (& mut out1) . index_mut (4) = (0x0 as u64) ; }
    };
}

fiat_25519_scalar_msat!()