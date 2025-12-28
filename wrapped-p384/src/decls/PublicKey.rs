macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! PublicKey {
    () => {
        deps!();
        # [doc = " NIST P-384 public key."] # [cfg (feature = "arithmetic")] pub type PublicKey = elliptic_curve :: PublicKey < NistP384 > ;
    };
}

PublicKey!();