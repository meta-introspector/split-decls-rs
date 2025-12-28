macro_rules! sc_sq {
    () => {
        # [cfg (feature = "blind-keys")] pub fn sc_sq (s : & [u8]) -> [u8 ; 32] { sc_mul (s , s) }
    };
}

sc_sq!()