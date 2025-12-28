macro_rules! tee {
    () => {
        # [cfg (feature = "use_alloc")] mod tee ;
    };
}

tee!()