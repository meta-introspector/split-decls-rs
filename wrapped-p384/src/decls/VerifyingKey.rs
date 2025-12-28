macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! VerifyingKey {
    () => {
        deps!();
        # [doc = " ECDSA/P-384 verification key (i.e. public key)"] # [cfg (feature = "ecdsa")] pub type VerifyingKey = ecdsa_core :: VerifyingKey < NistP384 > ;
    };
}

VerifyingKey!()