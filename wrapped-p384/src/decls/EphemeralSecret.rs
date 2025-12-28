macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! EphemeralSecret {
    () => {
        deps!();
        # [doc = " NIST P-384 Ephemeral Diffie-Hellman Secret."] pub type EphemeralSecret = elliptic_curve :: ecdh :: EphemeralSecret < NistP384 > ;
    };
}

EphemeralSecret!()