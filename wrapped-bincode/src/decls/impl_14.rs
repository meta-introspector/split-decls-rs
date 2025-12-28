macro_rules! deps {
    () => {
        EncodeError!();
        Encoder!();
        Encode!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "64")] impl Encode for AtomicU64 { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
    };
}

impl_14!();