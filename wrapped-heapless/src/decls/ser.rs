macro_rules! ser {
    () => {
        # [cfg (feature = "serde")] mod ser ;
    };
}

ser!();