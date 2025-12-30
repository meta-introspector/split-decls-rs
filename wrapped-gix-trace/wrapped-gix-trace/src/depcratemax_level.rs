// Generated macro for MAX_LEVEL (const)
macro_rules! DepcrateMAX_LEVEL {
() => {
// Module: crate
// Provides: {"MAX_LEVEL"}
// Dependencies: {}
# [doc = " The maximum allowed level for tracing items, as compiled in."] # [cfg (not (feature = "tracing-detail"))] pub const MAX_LEVEL : Level = Level :: Coarse ;
};
}
