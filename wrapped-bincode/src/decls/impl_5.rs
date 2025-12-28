macro_rules! deps {
    () => {
        EncodeError!();
        Encoder!();
        Encode!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "8")] impl Encode for AtomicU8 { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
    };
}

impl_5!();