macro_rules! logger {
    () => {
        # [cfg (feature = "log")] mod logger ;
    };
}

logger!();