macro_rules! boxed {
    () => {
        # [cfg (feature = "alloc")] pub mod boxed ;
    };
}

boxed!()