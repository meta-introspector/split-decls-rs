macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_25519_relax {
    () => {
        deps!();
        # [doc = " The function fiat_25519_relax is the identity function converting from tight field elements to loose field elements."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   out1 = arg1"] # [doc = ""] # [inline] pub const fn fiat_25519_relax (mut out1 : & mut fiat_25519_loose_field_element , arg1 : & fiat_25519_tight_field_element) { let x1 : u64 = (* IndexConst (arg1) . index (0)) ; let x2 : u64 = (* IndexConst (arg1) . index (1)) ; let x3 : u64 = (* IndexConst (arg1) . index (2)) ; let x4 : u64 = (* IndexConst (arg1) . index (3)) ; let x5 : u64 = (* IndexConst (arg1) . index (4)) ; * IndexConst (& mut out1) . index_mut (0) = x1 ; * IndexConst (& mut out1) . index_mut (1) = x2 ; * IndexConst (& mut out1) . index_mut (2) = x3 ; * IndexConst (& mut out1) . index_mut (3) = x4 ; * IndexConst (& mut out1) . index_mut (4) = x5 ; }
    };
}

fiat_25519_relax!()