macro_rules! deps {
    () => {
        EncodeError!();
        Encode!();
        Encoder!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        # [cfg (target_has_atomic = "ptr")] impl Encode for AtomicUsize { fn encode < E : crate :: enc :: Encoder > (& self , encoder : & mut E ,) -> Result < () , crate :: error :: EncodeError > { self . load (Ordering :: SeqCst) . encode (encoder) } }
    };
}

impl_17!();