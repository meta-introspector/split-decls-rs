macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! SigningKey {
    () => {
        deps!();
        # [doc = " ECDSA/P-384 signing key"] # [cfg (feature = "ecdsa")] pub type SigningKey = ecdsa_core :: SigningKey < NistP384 > ;
    };
}

SigningKey!()