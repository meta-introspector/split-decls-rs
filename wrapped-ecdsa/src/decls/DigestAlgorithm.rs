macro_rules! deps {
    () => {
        EcdsaCurve!();
    };
}

macro_rules! DigestAlgorithm {
    () => {
        deps!();
        # [doc = " Bind a preferred [`Digest`] algorithm to an elliptic curve type."] # [doc = ""] # [doc = " Generally there is a preferred variety of the SHA-2 family used with ECDSA"] # [doc = " for a particular elliptic curve."] # [cfg (feature = "digest")] pub trait DigestAlgorithm : EcdsaCurve { # [doc = " Preferred digest to use when computing ECDSA signatures for this"] # [doc = " elliptic curve. This is typically a member of the SHA-2 family."] type Digest : EagerHash + digest :: Update ; }
    };
}

DigestAlgorithm!();