macro_rules! unique_impl {
    () => {
        # [cfg (feature = "use_std")] mod unique_impl ;
    };
}

unique_impl!();