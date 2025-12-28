macro_rules! impl_std {
    () => {
        # [cfg (feature = "std")] mod impl_std ;
    };
}

impl_std!();