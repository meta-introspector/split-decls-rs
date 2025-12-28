macro_rules! impl_alloc {
    () => {
        # [cfg (feature = "alloc")] mod impl_alloc ;
    };
}

impl_alloc!();