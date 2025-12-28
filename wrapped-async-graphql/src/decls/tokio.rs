macro_rules! tokio {
    () => {
        # [cfg (feature = "tokio-sync")] mod tokio ;
    };
}

tokio!()