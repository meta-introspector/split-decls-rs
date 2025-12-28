macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! ScalarBits {
    () => {
        deps!();
        # [doc = " Bit representation of a NIST P-384 scalar field element."] # [cfg (feature = "bits")] pub type ScalarBits = elliptic_curve :: scalar :: ScalarBits < NistP384 > ;
    };
}

ScalarBits!()