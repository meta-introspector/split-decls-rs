macro_rules! lock {
    () => {
        # [cfg (feature = "alloc")] pub mod lock ;
    };
}

lock!()