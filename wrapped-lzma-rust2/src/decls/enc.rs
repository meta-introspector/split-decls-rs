macro_rules! enc {
    () => {
        # [cfg (feature = "encoder")] mod enc ;
    };
}

enc!();