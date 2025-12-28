macro_rules! deps {
    () => {
        Digest!();
        FixedOutputCore!();
        CoreProxy!();
        BufferKindUser!();
        HashMarker!();
        EagerHash!();
        UpdateCore!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < T > EagerHash for T where T : CoreProxy + BlockSizeUser + Digest , < T as CoreProxy > :: Core : HashMarker + UpdateCore + FixedOutputCore + BlockSizeUser < BlockSize = < Self as BlockSizeUser > :: BlockSize > + BufferKindUser < BufferKind = Eager > + Default + Clone , { type Core = T :: Core ; }
    };
}

impl_33!();