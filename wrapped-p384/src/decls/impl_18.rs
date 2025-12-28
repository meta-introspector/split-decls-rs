macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        # [cfg (not (feature = "arithmetic"))] impl elliptic_curve :: sec1 :: ValidatePublicKey for NistP384 { }
    };
}

impl_18!()