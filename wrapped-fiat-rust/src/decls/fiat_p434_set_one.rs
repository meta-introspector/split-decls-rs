macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_p434_set_one {
    () => {
        deps!();
        # [doc = " The function fiat_p434_set_one returns the field element one in the Montgomery domain."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) mod m = 1 mod m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [inline] pub const fn fiat_p434_set_one (mut out1 : & mut fiat_p434_montgomery_domain_field_element) { * IndexConst (& mut out1) . index_mut (0) = 0x742c ; * IndexConst (& mut out1) . index_mut (1) = (0x0 as u64) ; * IndexConst (& mut out1) . index_mut (2) = (0x0 as u64) ; * IndexConst (& mut out1) . index_mut (3) = 0xb90ff404fc000000 ; * IndexConst (& mut out1) . index_mut (4) = 0xd801a4fb559facd4 ; * IndexConst (& mut out1) . index_mut (5) = 0xe93254545f77410c ; * IndexConst (& mut out1) . index_mut (6) = 0xeceea7bd2eda ; }
    };
}

fiat_p434_set_one!();