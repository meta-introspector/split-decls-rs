macro_rules! pool {
    () => {
        # [cfg (feature = "alloc")] pub mod pool ;
    };
}

pool!()