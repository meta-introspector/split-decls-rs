macro_rules! deps {
    () => {
        SignatureSize!();
        SignatureWithOid!();
        Signature!();
        DigestAlgorithm!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        # [doc = " NOTE: this implementation assumes the default digest for the given elliptic"] # [doc = " curve as defined by [`hazmat::DigestAlgorithm`]."] # [doc = ""] # [doc = " When working with alternative digests, you will need to use e.g."] # [doc = " [`SignatureWithOid::new_with_digest`]."] # [cfg (all (feature = "digest" , feature = "hazmat"))] impl < C > TryFrom < & [u8] > for SignatureWithOid < C > where C : hazmat :: DigestAlgorithm , C :: Digest : AssociatedOid , SignatureSize < C > : ArraySize , { type Error = Error ; fn try_from (slice : & [u8]) -> Result < Self > { Self :: new (Signature :: < C > :: from_slice (slice) ? , C :: Digest :: OID) } }
    };
}

impl_169!()