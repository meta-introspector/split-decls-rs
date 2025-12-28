macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_25519_sub {
    () => {
        deps!();
        # [doc = " The function fiat_25519_sub subtracts two field elements."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval out1 mod m = (eval arg1 - eval arg2) mod m"] # [doc = ""] # [inline] pub const fn fiat_25519_sub (mut out1 : & mut fiat_25519_loose_field_element , arg1 : & fiat_25519_tight_field_element , arg2 : & fiat_25519_tight_field_element) { let x1 : u64 = ((0xfffffffffffda + (* IndexConst (arg1) . index (0))) - (* IndexConst (arg2) . index (0))) ; let x2 : u64 = ((0xffffffffffffe + (* IndexConst (arg1) . index (1))) - (* IndexConst (arg2) . index (1))) ; let x3 : u64 = ((0xffffffffffffe + (* IndexConst (arg1) . index (2))) - (* IndexConst (arg2) . index (2))) ; let x4 : u64 = ((0xffffffffffffe + (* IndexConst (arg1) . index (3))) - (* IndexConst (arg2) . index (3))) ; let x5 : u64 = ((0xffffffffffffe + (* IndexConst (arg1) . index (4))) - (* IndexConst (arg2) . index (4))) ; * IndexConst (& mut out1) . index_mut (0) = x1 ; * IndexConst (& mut out1) . index_mut (1) = x2 ; * IndexConst (& mut out1) . index_mut (2) = x3 ; * IndexConst (& mut out1) . index_mut (3) = x4 ; * IndexConst (& mut out1) . index_mut (4) = x5 ; }
    };
}

fiat_25519_sub!()