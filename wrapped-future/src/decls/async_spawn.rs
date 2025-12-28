macro_rules! async_spawn {
    () => {
        # [cfg (feature = "std")] mod async_spawn ;
    };
}

async_spawn!()