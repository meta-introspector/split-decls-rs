macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_p521_carry {
    () => {
        deps!();
        # [doc = " The function fiat_p521_carry reduces a field element."] # [doc = ""] # [doc = " Postconditions:"] # [doc = "   eval out1 mod m = eval arg1 mod m"] # [doc = ""] # [inline] pub const fn fiat_p521_carry (mut out1 : & mut fiat_p521_tight_field_element , arg1 : & fiat_p521_loose_field_element) { let x1 : u64 = (* IndexConst (arg1) . index (0)) ; let x2 : u64 = ((x1 >> 58) + (* IndexConst (arg1) . index (1))) ; let x3 : u64 = ((x2 >> 58) + (* IndexConst (arg1) . index (2))) ; let x4 : u64 = ((x3 >> 58) + (* IndexConst (arg1) . index (3))) ; let x5 : u64 = ((x4 >> 58) + (* IndexConst (arg1) . index (4))) ; let x6 : u64 = ((x5 >> 58) + (* IndexConst (arg1) . index (5))) ; let x7 : u64 = ((x6 >> 58) + (* IndexConst (arg1) . index (6))) ; let x8 : u64 = ((x7 >> 58) + (* IndexConst (arg1) . index (7))) ; let x9 : u64 = ((x8 >> 58) + (* IndexConst (arg1) . index (8))) ; let x10 : u64 = ((x1 & 0x3ffffffffffffff) + (x9 >> 57)) ; let x11 : u64 = ((((x10 >> 58) as fiat_p521_u1) as u64) + (x2 & 0x3ffffffffffffff)) ; let x12 : u64 = (x10 & 0x3ffffffffffffff) ; let x13 : u64 = (x11 & 0x3ffffffffffffff) ; let x14 : u64 = ((((x11 >> 58) as fiat_p521_u1) as u64) + (x3 & 0x3ffffffffffffff)) ; let x15 : u64 = (x4 & 0x3ffffffffffffff) ; let x16 : u64 = (x5 & 0x3ffffffffffffff) ; let x17 : u64 = (x6 & 0x3ffffffffffffff) ; let x18 : u64 = (x7 & 0x3ffffffffffffff) ; let x19 : u64 = (x8 & 0x3ffffffffffffff) ; let x20 : u64 = (x9 & 0x1ffffffffffffff) ; * IndexConst (& mut out1) . index_mut (0) = x12 ; * IndexConst (& mut out1) . index_mut (1) = x13 ; * IndexConst (& mut out1) . index_mut (2) = x14 ; * IndexConst (& mut out1) . index_mut (3) = x15 ; * IndexConst (& mut out1) . index_mut (4) = x16 ; * IndexConst (& mut out1) . index_mut (5) = x17 ; * IndexConst (& mut out1) . index_mut (6) = x18 ; * IndexConst (& mut out1) . index_mut (7) = x19 ; * IndexConst (& mut out1) . index_mut (8) = x20 ; }
    };
}

fiat_p521_carry!();