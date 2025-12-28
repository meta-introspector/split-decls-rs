macro_rules! deps {
    () => {
        DynDigest!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl Clone for Box < dyn DynDigest > { fn clone (& self) -> Self { self . box_clone () } }
    };
}

impl_51!()