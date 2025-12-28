macro_rules! sc_mul {
    () => {
        # [cfg (feature = "blind-keys")] pub fn sc_mul (a : & [u8] , b : & [u8]) -> [u8 ; 32] { let mut s = [0u8 ; 32] ; sc_muladd (& mut s , a , b , & [0 ; 32]) ; s }
    };
}

sc_mul!();