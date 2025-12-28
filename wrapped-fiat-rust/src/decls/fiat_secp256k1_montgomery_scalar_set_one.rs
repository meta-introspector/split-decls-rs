macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_secp256k1_montgomery_scalar_set_one {
    () => {
        deps!();
        # [doc = " The function fiat_secp256k1_montgomery_scalar_set_one returns the field element one in the Montgomery domain."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) mod m = 1 mod m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [inline] pub const fn fiat_secp256k1_montgomery_scalar_set_one (mut out1 : & mut fiat_secp256k1_montgomery_scalar_montgomery_domain_field_element) { * IndexConst (& mut out1) . index_mut (0) = 0x402da1732fc9bebf ; * IndexConst (& mut out1) . index_mut (1) = 0x4551231950b75fc4 ; * IndexConst (& mut out1) . index_mut (2) = (0x1 as u64) ; * IndexConst (& mut out1) . index_mut (3) = (0x0 as u64) ; }
    };
}

fiat_secp256k1_montgomery_scalar_set_one!()