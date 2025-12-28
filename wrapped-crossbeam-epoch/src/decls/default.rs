macro_rules! default {
    () => {
        # [cfg (feature = "std")] mod default ;
    };
}

default!();