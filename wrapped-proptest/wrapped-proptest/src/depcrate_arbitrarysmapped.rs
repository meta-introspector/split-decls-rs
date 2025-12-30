// Generated macro for SMapped (type)
macro_rules! Depcrate_arbitrarySMapped {
() => {
// Module: crate::arbitrary
// Provides: {"SMapped"}
// Dependencies: {}
# [doc = " A static map from a strategy of `I` to `O`."] # [doc = ""] # [doc = " # Stability"] # [doc = ""] # [doc = " This is provided to make documentation more readable."] # [doc = " Do not rely on it existing in your own code."] pub type SMapped < I , O > = statics :: Map < StrategyFor < I > , fn (I) -> O > ;
};
}
