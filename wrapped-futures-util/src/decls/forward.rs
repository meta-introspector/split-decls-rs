macro_rules! forward {
    () => {
        # [cfg (feature = "sink")] mod forward ;
    };
}

forward!();