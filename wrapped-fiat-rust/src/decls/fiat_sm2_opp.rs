macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_sm2_opp {
    () => {
        deps!();
        # [doc = " The function fiat_sm2_opp negates a field element in the Montgomery domain."] # [doc = ""] # [doc = " Preconditions:"] # [doc = "   0 ≤ eval arg1 < m"] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) mod m = -eval (from_montgomery arg1) mod m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [inline] pub const fn fiat_sm2_opp (mut out1 : & mut fiat_sm2_montgomery_domain_field_element , arg1 : & fiat_sm2_montgomery_domain_field_element) { let mut x1 : u64 = 0 ; let mut x2 : fiat_sm2_u1 = 0 ; fiat_sm2_subborrowx_u64 (& mut x1 , & mut x2 , 0x0 , (0x0 as u64) , (* IndexConst (arg1) . index (0))) ; let mut x3 : u64 = 0 ; let mut x4 : fiat_sm2_u1 = 0 ; fiat_sm2_subborrowx_u64 (& mut x3 , & mut x4 , x2 , (0x0 as u64) , (* IndexConst (arg1) . index (1))) ; let mut x5 : u64 = 0 ; let mut x6 : fiat_sm2_u1 = 0 ; fiat_sm2_subborrowx_u64 (& mut x5 , & mut x6 , x4 , (0x0 as u64) , (* IndexConst (arg1) . index (2))) ; let mut x7 : u64 = 0 ; let mut x8 : fiat_sm2_u1 = 0 ; fiat_sm2_subborrowx_u64 (& mut x7 , & mut x8 , x6 , (0x0 as u64) , (* IndexConst (arg1) . index (3))) ; let mut x9 : u64 = 0 ; fiat_sm2_cmovznz_u64 (& mut x9 , x8 , (0x0 as u64) , 0xffffffffffffffff) ; let mut x10 : u64 = 0 ; let mut x11 : fiat_sm2_u1 = 0 ; fiat_sm2_addcarryx_u64 (& mut x10 , & mut x11 , 0x0 , x1 , x9) ; let mut x12 : u64 = 0 ; let mut x13 : fiat_sm2_u1 = 0 ; fiat_sm2_addcarryx_u64 (& mut x12 , & mut x13 , x11 , x3 , (x9 & 0xffffffff00000000)) ; let mut x14 : u64 = 0 ; let mut x15 : fiat_sm2_u1 = 0 ; fiat_sm2_addcarryx_u64 (& mut x14 , & mut x15 , x13 , x5 , x9) ; let mut x16 : u64 = 0 ; let mut x17 : fiat_sm2_u1 = 0 ; fiat_sm2_addcarryx_u64 (& mut x16 , & mut x17 , x15 , x7 , (x9 & 0xfffffffeffffffff)) ; * IndexConst (& mut out1) . index_mut (0) = x10 ; * IndexConst (& mut out1) . index_mut (1) = x12 ; * IndexConst (& mut out1) . index_mut (2) = x14 ; * IndexConst (& mut out1) . index_mut (3) = x16 ; }
    };
}

fiat_sm2_opp!();