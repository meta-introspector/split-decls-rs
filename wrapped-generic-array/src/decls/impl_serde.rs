macro_rules! impl_serde {
    () => {
        # [cfg (feature = "serde")] mod impl_serde ;
    };
}

impl_serde!();