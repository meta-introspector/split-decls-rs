macro_rules! deps {
    () => {
        ReseedingRng!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl < R , Rsdr > CryptoRng for ReseedingRng < R , Rsdr > where R : BlockRngCore < Item = u32 > + SeedableRng + CryptoBlockRng , Rsdr : TryCryptoRng , { }
    };
}

impl_221!()