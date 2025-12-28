macro_rules! deps {
    () => {
        ReseedingCore!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < R , Rsdr > BlockRngCore for ReseedingCore < R , Rsdr > where R : BlockRngCore + SeedableRng , Rsdr : TryRngCore , { type Item = < R as BlockRngCore > :: Item ; type Results = < R as BlockRngCore > :: Results ; fn generate (& mut self , results : & mut Self :: Results) { if self . bytes_until_reseed <= 0 { return self . reseed_and_generate (results) ; } let num_bytes = size_of_val (results . as_ref ()) ; self . bytes_until_reseed -= num_bytes as i64 ; self . inner . generate (results) ; } }
    };
}

impl_223!()