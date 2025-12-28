macro_rules! impl_zeroize {
    () => {
        # [cfg (feature = "zeroize")] mod impl_zeroize ;
    };
}

impl_zeroize!();