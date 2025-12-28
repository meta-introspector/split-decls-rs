macro_rules! deps {
    () => {
        RingElement!();
        RingElementNTT!();
        FieldElement!();
    };
}

macro_rules! inverse_ntt {
    () => {
        deps!();
        pub fn inverse_ntt (f_hat : & RingElementNTT) -> RingElement { let mut f = RingElement :: copy_from_ntt (f_hat) ; let mut len = 2 ; let mut i = 127 ; while len <= 128 { let mut start = 0 ; while start < 256 { let zeta = FieldElement :: new (ZETA_ALL [i]) ; i -= 1 ; for j in start .. (start + len) { let t : FieldElement = f [j] ; f [j] = t + f [j + len] ; f [j + len] = zeta * (f [j + len] - t) ; } start += 2 * len ; } len *= 2 ; } for fe in f . coefficients . iter_mut () { * fe = * fe * FieldElement (3303) ; } f }
    };
}

inverse_ntt!();