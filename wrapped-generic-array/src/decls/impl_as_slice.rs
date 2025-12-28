macro_rules! impl_as_slice {
    () => {
        # [cfg (feature = "as_slice")] mod impl_as_slice ;
    };
}

impl_as_slice!()