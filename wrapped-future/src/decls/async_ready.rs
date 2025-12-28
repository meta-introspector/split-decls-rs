macro_rules! async_ready {
    () => {
        # [cfg (feature = "std")] mod async_ready ;
    };
}

async_ready!();