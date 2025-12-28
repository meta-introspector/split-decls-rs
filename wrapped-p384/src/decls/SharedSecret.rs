macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! SharedSecret {
    () => {
        deps!();
        # [doc = " Shared secret value computed via ECDH key agreement."] pub type SharedSecret = elliptic_curve :: ecdh :: SharedSecret < NistP384 > ;
    };
}

SharedSecret!();