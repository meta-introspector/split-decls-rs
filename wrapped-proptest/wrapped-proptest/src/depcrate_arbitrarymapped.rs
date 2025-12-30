// Generated macro for Mapped (type)
macro_rules! Depcrate_arbitraryMapped {
() => {
// Module: crate::arbitrary
// Provides: {"Mapped"}
// Dependencies: {}
# [doc = " A normal map from a strategy of `I` to `O`."] # [doc = ""] # [doc = " # Stability"] # [doc = ""] # [doc = " This is provided to make documentation more readable."] # [doc = " Do not rely on it existing in your own code."] pub type Mapped < I , O > = Map < StrategyFor < I > , fn (I) -> O > ;
};
}
