macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_25519_carry {
    () => {
        deps!();
        # [doc = " The function fiat_25519_carry reduces a field element."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval out1 mod m = eval arg1 mod m"] # [doc = ""] # [inline] pub const fn fiat_25519_carry (mut out1 : & mut fiat_25519_tight_field_element , arg1 : & fiat_25519_loose_field_element) { let x1 : u64 = (* IndexConst (arg1) . index (0)) ; let x2 : u64 = ((x1 >> 51) + (* IndexConst (arg1) . index (1))) ; let x3 : u64 = ((x2 >> 51) + (* IndexConst (arg1) . index (2))) ; let x4 : u64 = ((x3 >> 51) + (* IndexConst (arg1) . index (3))) ; let x5 : u64 = ((x4 >> 51) + (* IndexConst (arg1) . index (4))) ; let x6 : u64 = ((x1 & 0x7ffffffffffff) + ((x5 >> 51) * 0x13)) ; let x7 : u64 = ((((x6 >> 51) as fiat_25519_u1) as u64) + (x2 & 0x7ffffffffffff)) ; let x8 : u64 = (x6 & 0x7ffffffffffff) ; let x9 : u64 = (x7 & 0x7ffffffffffff) ; let x10 : u64 = ((((x7 >> 51) as fiat_25519_u1) as u64) + (x3 & 0x7ffffffffffff)) ; let x11 : u64 = (x4 & 0x7ffffffffffff) ; let x12 : u64 = (x5 & 0x7ffffffffffff) ; * IndexConst (& mut out1) . index_mut (0) = x8 ; * IndexConst (& mut out1) . index_mut (1) = x9 ; * IndexConst (& mut out1) . index_mut (2) = x10 ; * IndexConst (& mut out1) . index_mut (3) = x11 ; * IndexConst (& mut out1) . index_mut (4) = x12 ; }
    };
}

fiat_25519_carry!();