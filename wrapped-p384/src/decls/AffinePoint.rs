macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! AffinePoint {
    () => {
        deps!();
        # [doc = " Elliptic curve point in affine coordinates."] pub type AffinePoint = primeorder :: AffinePoint < NistP384 > ;
    };
}

AffinePoint!()