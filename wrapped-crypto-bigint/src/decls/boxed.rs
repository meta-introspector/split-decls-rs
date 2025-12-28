macro_rules! boxed {
    () => {
        # [cfg (feature = "alloc")] pub (crate) mod boxed ;
    };
}

boxed!()