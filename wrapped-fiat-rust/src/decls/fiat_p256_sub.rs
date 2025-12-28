macro_rules! deps {
    () => {
        IndexConst!();
    };
}

macro_rules! fiat_p256_sub {
    () => {
        deps!();
        # [doc = " The function fiat_p256_sub subtracts two field elements in the Montgomery domain."] # [doc = ""] # [doc = " Preconditions:"] # [doc = "   0 ≤ eval arg1 < m"] # [doc = "   0 ≤ eval arg2 < m"] # [doc = " Postconditions:"] # [doc = "   eval (from_montgomery out1) mod m = (eval (from_montgomery arg1) - eval (from_montgomery arg2)) mod m"] # [doc = "   0 ≤ eval out1 < m"] # [doc = ""] # [inline] pub const fn fiat_p256_sub (mut out1 : & mut fiat_p256_montgomery_domain_field_element , arg1 : & fiat_p256_montgomery_domain_field_element , arg2 : & fiat_p256_montgomery_domain_field_element) { let mut x1 : u64 = 0 ; let mut x2 : fiat_p256_u1 = 0 ; fiat_p256_subborrowx_u64 (& mut x1 , & mut x2 , 0x0 , (* IndexConst (arg1) . index (0)) , (* IndexConst (arg2) . index (0))) ; let mut x3 : u64 = 0 ; let mut x4 : fiat_p256_u1 = 0 ; fiat_p256_subborrowx_u64 (& mut x3 , & mut x4 , x2 , (* IndexConst (arg1) . index (1)) , (* IndexConst (arg2) . index (1))) ; let mut x5 : u64 = 0 ; let mut x6 : fiat_p256_u1 = 0 ; fiat_p256_subborrowx_u64 (& mut x5 , & mut x6 , x4 , (* IndexConst (arg1) . index (2)) , (* IndexConst (arg2) . index (2))) ; let mut x7 : u64 = 0 ; let mut x8 : fiat_p256_u1 = 0 ; fiat_p256_subborrowx_u64 (& mut x7 , & mut x8 , x6 , (* IndexConst (arg1) . index (3)) , (* IndexConst (arg2) . index (3))) ; let mut x9 : u64 = 0 ; fiat_p256_cmovznz_u64 (& mut x9 , x8 , (0x0 as u64) , 0xffffffffffffffff) ; let mut x10 : u64 = 0 ; let mut x11 : fiat_p256_u1 = 0 ; fiat_p256_addcarryx_u64 (& mut x10 , & mut x11 , 0x0 , x1 , x9) ; let mut x12 : u64 = 0 ; let mut x13 : fiat_p256_u1 = 0 ; fiat_p256_addcarryx_u64 (& mut x12 , & mut x13 , x11 , x3 , (x9 & 0xffffffff)) ; let mut x14 : u64 = 0 ; let mut x15 : fiat_p256_u1 = 0 ; fiat_p256_addcarryx_u64 (& mut x14 , & mut x15 , x13 , x5 , (0x0 as u64)) ; let mut x16 : u64 = 0 ; let mut x17 : fiat_p256_u1 = 0 ; fiat_p256_addcarryx_u64 (& mut x16 , & mut x17 , x15 , x7 , (x9 & 0xffffffff00000001)) ; * IndexConst (& mut out1) . index_mut (0) = x10 ; * IndexConst (& mut out1) . index_mut (1) = x12 ; * IndexConst (& mut out1) . index_mut (2) = x14 ; * IndexConst (& mut out1) . index_mut (3) = x16 ; }
    };
}

fiat_p256_sub!();