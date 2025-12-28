macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! MAX_LEVEL {
    () => {
        deps!();
        # [doc = " The maximum allowed level for tracing items, as compiled in."] # [cfg (not (feature = "tracing-detail"))] pub const MAX_LEVEL : Level = Level :: Coarse ;
    };
}

MAX_LEVEL!();