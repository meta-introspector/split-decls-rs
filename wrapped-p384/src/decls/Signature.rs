macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! Signature {
    () => {
        deps!();
        # [doc = " ECDSA/P-384 signature (fixed-size)"] pub type Signature = ecdsa_core :: Signature < NistP384 > ;
    };
}

Signature!();