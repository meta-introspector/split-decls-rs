macro_rules! impl_subtle {
    () => {
        # [cfg (feature = "subtle")] mod impl_subtle ;
    };
}

impl_subtle!();