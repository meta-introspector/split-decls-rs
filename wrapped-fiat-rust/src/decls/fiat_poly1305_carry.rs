macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_poly1305_carry {
    () => {
        deps!();
        # [doc = " The function fiat_poly1305_carry reduces a field element."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval out1 mod m = eval arg1 mod m"] # [doc = ""] # [inline] pub const fn fiat_poly1305_carry (mut out1 : & mut fiat_poly1305_tight_field_element , arg1 : & fiat_poly1305_loose_field_element) { let x1 : u64 = (* IndexConst (arg1) . index (0)) ; let x2 : u64 = ((x1 >> 44) + (* IndexConst (arg1) . index (1))) ; let x3 : u64 = ((x2 >> 43) + (* IndexConst (arg1) . index (2))) ; let x4 : u64 = ((x1 & 0xfffffffffff) + ((x3 >> 43) * 0x5)) ; let x5 : u64 = ((((x4 >> 44) as fiat_poly1305_u1) as u64) + (x2 & 0x7ffffffffff)) ; let x6 : u64 = (x4 & 0xfffffffffff) ; let x7 : u64 = (x5 & 0x7ffffffffff) ; let x8 : u64 = ((((x5 >> 43) as fiat_poly1305_u1) as u64) + (x3 & 0x7ffffffffff)) ; * IndexConst (& mut out1) . index_mut (0) = x6 ; * IndexConst (& mut out1) . index_mut (1) = x7 ; * IndexConst (& mut out1) . index_mut (2) = x8 ; }
    };
}

fiat_poly1305_carry!();