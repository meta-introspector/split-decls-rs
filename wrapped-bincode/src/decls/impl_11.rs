macro_rules! deps {
    () => {
        Encoder!();
        Encode!();
        EncodeError!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "32")] impl Encode for AtomicU32 { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
    };
}

impl_11!();