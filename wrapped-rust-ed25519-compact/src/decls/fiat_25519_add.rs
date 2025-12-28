macro_rules! fiat_25519_add {
    () => {
        # [cfg_attr (feature = "opt_size" , inline (never))] # [cfg_attr (not (feature = "opt_size") , inline)] pub fn fiat_25519_add (out1 : & mut [u64 ; 5] , arg1 : & [u64 ; 5] , arg2 : & [u64 ; 5]) { let x1 : u64 = ((arg1 [0]) . wrapping_add ((arg2 [0]))) ; let x2 : u64 = ((arg1 [1]) . wrapping_add ((arg2 [1]))) ; let x3 : u64 = ((arg1 [2]) . wrapping_add ((arg2 [2]))) ; let x4 : u64 = ((arg1 [3]) . wrapping_add ((arg2 [3]))) ; let x5 : u64 = ((arg1 [4]) . wrapping_add ((arg2 [4]))) ; out1 [0] = x1 ; out1 [1] = x2 ; out1 [2] = x3 ; out1 [3] = x4 ; out1 [4] = x5 ; }
    };
}

fiat_25519_add!();