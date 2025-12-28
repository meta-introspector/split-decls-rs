macro_rules! fiat_25519_cmovznz_u64 {
    () => {
        # [cfg_attr (feature = "opt_size" , inline (never))] # [cfg_attr (not (feature = "opt_size") , inline)] pub fn fiat_25519_cmovznz_u64 (out1 : & mut u64 , arg1 : fiat_25519_u1 , arg2 : u64 , arg3 : u64) { let x1 : fiat_25519_u1 = (! (! arg1)) ; let x2 : u64 = (((((0x0_i8 . wrapping_sub ((x1 as fiat_25519_i2))) as fiat_25519_i1) as i128) & 0xffffffffffffffff_i128) as u64) ; let x3 : u64 = ((x2 & arg3) | ((! x2) & arg2)) ; * out1 = x3 ; }
    };
}

fiat_25519_cmovznz_u64!()