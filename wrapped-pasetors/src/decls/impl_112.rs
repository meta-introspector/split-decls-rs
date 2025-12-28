macro_rules! deps {
    () => {
        V4!();
        Error!();
        SymmetricKey!();
        Generate!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl Generate < SymmetricKey < V4 > , V4 > for SymmetricKey < V4 > { fn generate () -> Result < SymmetricKey < V4 > , Error > { let mut rng_bytes = vec ! [0u8 ; V4 :: LOCAL_KEY] ; V4 :: validate_local_key (& rng_bytes) ? ; getrandom :: fill (& mut rng_bytes) ? ; Ok (Self { bytes : rng_bytes , phantom : PhantomData , }) } }
    };
}

impl_112!()