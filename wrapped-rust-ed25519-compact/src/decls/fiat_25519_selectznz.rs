macro_rules! fiat_25519_selectznz {
    () => {
        # [cfg_attr (feature = "opt_size" , inline (never))] # [cfg_attr (not (feature = "opt_size") , inline)] pub fn fiat_25519_selectznz (out1 : & mut [u64 ; 5] , arg1 : fiat_25519_u1 , arg2 : & [u64 ; 5] , arg3 : & [u64 ; 5] ,) { let mut x1 : u64 = 0 ; fiat_25519_cmovznz_u64 (& mut x1 , arg1 , (arg2 [0]) , (arg3 [0])) ; let mut x2 : u64 = 0 ; fiat_25519_cmovznz_u64 (& mut x2 , arg1 , (arg2 [1]) , (arg3 [1])) ; let mut x3 : u64 = 0 ; fiat_25519_cmovznz_u64 (& mut x3 , arg1 , (arg2 [2]) , (arg3 [2])) ; let mut x4 : u64 = 0 ; fiat_25519_cmovznz_u64 (& mut x4 , arg1 , (arg2 [3]) , (arg3 [3])) ; let mut x5 : u64 = 0 ; fiat_25519_cmovznz_u64 (& mut x5 , arg1 , (arg2 [4]) , (arg3 [4])) ; out1 [0] = x1 ; out1 [1] = x2 ; out1 [2] = x3 ; out1 [3] = x4 ; out1 [4] = x5 ; }
    };
}

fiat_25519_selectznz!();