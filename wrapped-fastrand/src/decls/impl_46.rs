macro_rules! deps {
    () => {
        Rng!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl Rng { # [doc = " Generates a random `u32`."] # [inline] fn gen_u32 (& mut self) -> u32 { self . gen_u64 () as u32 } # [doc = " Generates a random `u64`."] # [inline] fn gen_u64 (& mut self) -> u64 { const WY_CONST_0 : u64 = 0x2d35_8dcc_aa6c_78a5 ; const WY_CONST_1 : u64 = 0x8bb8_4b93_962e_acc9 ; let s = self . 0 . wrapping_add (WY_CONST_0) ; self . 0 = s ; let t = u128 :: from (s) * u128 :: from (s ^ WY_CONST_1) ; (t as u64) ^ (t >> 64) as u64 } # [doc = " Generates a random `u128`."] # [inline] fn gen_u128 (& mut self) -> u128 { (u128 :: from (self . gen_u64 ()) << 64) | u128 :: from (self . gen_u64 ()) } # [doc = " Generates a random `u32` in `0..n`."] # [inline] fn gen_mod_u32 (& mut self , n : u32) -> u32 { let mut r = self . gen_u32 () ; let mut hi = mul_high_u32 (r , n) ; let mut lo = r . wrapping_mul (n) ; if lo < n { let t = n . wrapping_neg () % n ; while lo < t { r = self . gen_u32 () ; hi = mul_high_u32 (r , n) ; lo = r . wrapping_mul (n) ; } } hi } # [doc = " Generates a random `u64` in `0..n`."] # [inline] fn gen_mod_u64 (& mut self , n : u64) -> u64 { let mut r = self . gen_u64 () ; let mut hi = mul_high_u64 (r , n) ; let mut lo = r . wrapping_mul (n) ; if lo < n { let t = n . wrapping_neg () % n ; while lo < t { r = self . gen_u64 () ; hi = mul_high_u64 (r , n) ; lo = r . wrapping_mul (n) ; } } hi } # [doc = " Generates a random `u128` in `0..n`."] # [inline] fn gen_mod_u128 (& mut self , n : u128) -> u128 { let mut r = self . gen_u128 () ; let mut hi = mul_high_u128 (r , n) ; let mut lo = r . wrapping_mul (n) ; if lo < n { let t = n . wrapping_neg () % n ; while lo < t { r = self . gen_u128 () ; hi = mul_high_u128 (r , n) ; lo = r . wrapping_mul (n) ; } } hi } }
    };
}

impl_46!()