macro_rules! impl_arbitrary {
    () => {
        # [cfg (feature = "arbitrary")] mod impl_arbitrary ;
    };
}

impl_arbitrary!();