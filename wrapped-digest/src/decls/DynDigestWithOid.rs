macro_rules! deps {
    () => {
        DynDigest!();
    };
}

macro_rules! DynDigestWithOid {
    () => {
        deps!();
        # [doc = " Convenience wrapper trait around [DynDigest] and [DynAssociatedOid]."] # [cfg (feature = "const-oid")] pub trait DynDigestWithOid : DynDigest + DynAssociatedOid { }
    };
}

DynDigestWithOid!()