macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_p256_set_one {
    () => {
        deps!();
        # [doc = " The function fiat_p256_set_one returns the field element one in the Montgomery domain."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) mod m = 1 mod m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [inline] pub const fn fiat_p256_set_one (mut out1 : & mut fiat_p256_montgomery_domain_field_element) { * IndexConst (& mut out1) . index_mut (0) = (0x1 as u64) ; * IndexConst (& mut out1) . index_mut (1) = 0xffffffff00000000 ; * IndexConst (& mut out1) . index_mut (2) = 0xffffffffffffffff ; * IndexConst (& mut out1) . index_mut (3) = 0xfffffffe ; }
    };
}

fiat_p256_set_one!()