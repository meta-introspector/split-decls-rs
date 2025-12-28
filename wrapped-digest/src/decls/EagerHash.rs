macro_rules! deps {
    () => {
        HashMarker!();
        UpdateCore!();
        BufferKindUser!();
        FixedOutputCore!();
        Digest!();
    };
}

macro_rules! EagerHash {
    () => {
        deps!();
        # [doc = " Trait implemented by eager hashes which expose their block-level core."] pub trait EagerHash : BlockSizeUser + Digest { # [doc = " Block-level core type of the hash."] type Core : HashMarker + UpdateCore + FixedOutputCore + BlockSizeUser < BlockSize = < Self as BlockSizeUser > :: BlockSize > + BufferKindUser < BufferKind = Eager > + Default + Clone ; }
    };
}

EagerHash!();