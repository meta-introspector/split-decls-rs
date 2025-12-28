macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        # [cfg (not (feature = "arithmetic"))] impl elliptic_curve :: sec1 :: ValidatePublicKey for NistP384 { }
    };
}

impl_44!();