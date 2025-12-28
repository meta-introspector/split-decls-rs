macro_rules! deps {
    () => {
        DynDigest!();
        DynDigestWithOid!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        # [cfg (feature = "const-oid")] impl < T : DynDigest + DynAssociatedOid > DynDigestWithOid for T { }
    };
}

impl_53!();