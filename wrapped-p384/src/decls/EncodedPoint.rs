macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! EncodedPoint {
    () => {
        deps!();
        # [doc = " NIST P-384 SEC1 encoded point."] pub type EncodedPoint = elliptic_curve :: sec1 :: EncodedPoint < NistP384 > ;
    };
}

EncodedPoint!()