macro_rules! deps {
    () => {
        GeP3!();
    };
}

macro_rules! ge_scalarmult {
    () => {
        deps!();
        pub fn ge_scalarmult (scalar : & [u8] , base : & GeP3) -> GeP3 { let pc = ge_precompute (base) ; let mut q = GeP3 :: zero () ; let mut pos = 252 ; loop { let slot = ((scalar [pos >> 3] >> (pos & 7)) & 15) as usize ; let mut t = pc [0] ; for i in 1 .. 16 { t . maybe_set (& pc [i] , (((slot ^ i) . wrapping_sub (1)) >> 8) as u8 & 1) ; } q = q . add (t) . to_p3 () ; if pos == 0 { break ; } q = q . dbl () . to_p3 () . dbl () . to_p3 () . dbl () . to_p3 () . dbl () . to_p3 () ; pos -= 4 ; } q }
    };
}

ge_scalarmult!()