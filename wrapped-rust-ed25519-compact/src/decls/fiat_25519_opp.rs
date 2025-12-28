macro_rules! fiat_25519_opp {
    () => {
        # [cfg_attr (feature = "opt_size" , inline (never))] # [cfg_attr (not (feature = "opt_size") , inline)] pub fn fiat_25519_opp (out1 : & mut [u64 ; 5] , arg1 : & [u64 ; 5]) { let x1 : u64 = (0xfffffffffffdau64 . wrapping_sub ((arg1 [0]))) ; let x2 : u64 = (0xffffffffffffeu64 . wrapping_sub ((arg1 [1]))) ; let x3 : u64 = (0xffffffffffffeu64 . wrapping_sub ((arg1 [2]))) ; let x4 : u64 = (0xffffffffffffeu64 . wrapping_sub ((arg1 [3]))) ; let x5 : u64 = (0xffffffffffffeu64 . wrapping_sub ((arg1 [4]))) ; out1 [0] = x1 ; out1 [1] = x2 ; out1 [2] = x3 ; out1 [3] = x4 ; out1 [4] = x5 ; }
    };
}

fiat_25519_opp!();