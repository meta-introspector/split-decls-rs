macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! ProjectivePoint {
    () => {
        deps!();
        # [doc = " Elliptic curve point in projective coordinates."] pub type ProjectivePoint = primeorder :: ProjectivePoint < NistP384 > ;
    };
}

ProjectivePoint!()