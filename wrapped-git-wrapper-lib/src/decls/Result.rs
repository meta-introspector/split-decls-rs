macro_rules! Result {
    () => {
        # [cfg (not (feature = "anyhow_enabled"))] type Result < T > = std :: result :: Result < T , Box < dyn Error > > ;
    };
}

Result!();