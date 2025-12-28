macro_rules! try_forward {
    () => {
        # [cfg (feature = "sink")] mod try_forward ;
    };
}

try_forward!();