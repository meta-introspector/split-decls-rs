macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! SecretKey {
    () => {
        deps!();
        # [doc = " NIST P-384 secret key."] pub type SecretKey = elliptic_curve :: SecretKey < NistP384 > ;
    };
}

SecretKey!()