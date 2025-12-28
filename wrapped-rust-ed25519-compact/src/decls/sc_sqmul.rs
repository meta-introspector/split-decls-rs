macro_rules! sc_sqmul {
    () => {
        # [cfg (feature = "blind-keys")] pub fn sc_sqmul (s : & [u8] , n : usize , a : & [u8]) -> [u8 ; 32] { let mut t = [0u8 ; 32] ; t . copy_from_slice (s) ; for _ in 0 .. n { t = sc_sq (& t) ; } sc_mul (& t , a) }
    };
}

sc_sqmul!();