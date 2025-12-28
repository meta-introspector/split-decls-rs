macro_rules! local_pool {
    () => {
        # [cfg (feature = "std")] mod local_pool ;
    };
}

local_pool!();