macro_rules! deps {
    () => {
        Encode!();
        Encoder!();
        EncodeError!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "64")] impl Encode for AtomicI64 { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
    };
}

impl_29!();