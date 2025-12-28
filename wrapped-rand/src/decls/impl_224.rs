macro_rules! deps {
    () => {
        ReseedingCore!();
        Error!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl < R , Rsdr > ReseedingCore < R , Rsdr > where R : BlockRngCore + SeedableRng , Rsdr : TryRngCore , { # [doc = " Create a new `ReseedingCore`."] # [doc = ""] # [doc = " `threshold` is the maximum number of bytes produced by"] # [doc = " [`BlockRngCore::generate`] before attempting reseeding."] fn new (threshold : u64 , mut reseeder : Rsdr) -> Result < Self , Rsdr :: Error > { let threshold = if threshold == 0 { i64 :: MAX } else if threshold <= i64 :: MAX as u64 { threshold as i64 } else { i64 :: MAX } ; let inner = R :: try_from_rng (& mut reseeder) ? ; Ok (ReseedingCore { inner , reseeder , threshold , bytes_until_reseed : threshold , }) } # [doc = " Reseed the internal PRNG."] fn reseed (& mut self) -> Result < () , Rsdr :: Error > { R :: try_from_rng (& mut self . reseeder) . map (| result | { self . bytes_until_reseed = self . threshold ; self . inner = result }) } # [inline (never)] fn reseed_and_generate (& mut self , results : & mut < Self as BlockRngCore > :: Results) { trace ! ("Reseeding RNG (periodic reseed)") ; let num_bytes = size_of_val (results . as_ref ()) ; if let Err (e) = self . reseed () { warn ! ("Reseeding RNG failed: {}" , e) ; let _ = e ; } self . bytes_until_reseed = self . threshold - num_bytes as i64 ; self . inner . generate (results) ; } }
    };
}

impl_224!()