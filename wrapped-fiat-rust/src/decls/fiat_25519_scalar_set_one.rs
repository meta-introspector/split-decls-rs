macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_25519_scalar_set_one {
    () => {
        deps!();
        # [doc = " The function fiat_25519_scalar_set_one returns the field element one in the Montgomery domain."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) mod m = 1 mod m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [inline] pub const fn fiat_25519_scalar_set_one (mut out1 : & mut fiat_25519_scalar_montgomery_domain_field_element) { * IndexConst (& mut out1) . index_mut (0) = 0xd6ec31748d98951d ; * IndexConst (& mut out1) . index_mut (1) = 0xc6ef5bf4737dcf70 ; * IndexConst (& mut out1) . index_mut (2) = 0xfffffffffffffffe ; * IndexConst (& mut out1) . index_mut (3) = 0xfffffffffffffff ; }
    };
}

fiat_25519_scalar_set_one!();