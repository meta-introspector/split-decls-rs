macro_rules! deps {
    () => {
        V2!();
        Error!();
        Generate!();
        SymmetricKey!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl Generate < SymmetricKey < V2 > , V2 > for SymmetricKey < V2 > { fn generate () -> Result < SymmetricKey < V2 > , Error > { let mut rng_bytes = vec ! [0u8 ; V2 :: LOCAL_KEY] ; V2 :: validate_local_key (& rng_bytes) ? ; getrandom :: fill (& mut rng_bytes) ? ; Ok (Self { bytes : rng_bytes , phantom : PhantomData , }) } }
    };
}

impl_83!();