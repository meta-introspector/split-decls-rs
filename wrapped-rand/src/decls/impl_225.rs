macro_rules! deps {
    () => {
        ReseedingCore!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        impl < R , Rsdr > CryptoBlockRng for ReseedingCore < R , Rsdr > where R : BlockRngCore < Item = u32 > + SeedableRng + CryptoBlockRng , Rsdr : TryCryptoRng , { }
    };
}

impl_225!()