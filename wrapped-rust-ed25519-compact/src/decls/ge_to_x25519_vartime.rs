macro_rules! deps {
    () => {
        GeP3!();
    };
}

macro_rules! ge_to_x25519_vartime {
    () => {
        deps!();
        # [cfg (feature = "x25519")] pub fn ge_to_x25519_vartime (s : & [u8 ; 32]) -> Option < [u8 ; 32] > { let p = GeP3 :: from_bytes_vartime (s) ? ; let yed = p . y ; let x_mont = (FE_ONE + yed) * ((FE_ONE - yed) . invert ()) ; Some (x_mont . to_bytes ()) }
    };
}

ge_to_x25519_vartime!()