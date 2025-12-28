macro_rules! serde_impl {
    () => {
        # [cfg (feature = "serde")] mod serde_impl ;
    };
}

serde_impl!()