// Generated macro for impl_204 (impl)
macro_rules! Depcrate_unitimpl_204 {
() => {
// Module: crate::unit
// Provides: {"impl_204"}
// Dependencies: {}
# [doc = " Display and utilities"] impl Unit { # [doc = " Create a representation of `self` implementing [`Display`][std::fmt::Display] in configurable fashion."] # [doc = ""] # [doc = " * `current_value` is the progress value to display."] # [doc = " * `upper_bound` is the possibly available upper bound of `current_value`."] # [doc = " * `throughput` configures how throughput should be displayed if already available."] # [doc = ""] # [doc = " Note that `throughput` is usually not available the first time a value is displayed."] pub fn display (& self , current_value : Step , upper_bound : Option < Step > , throughput : impl Into < Option < display :: Throughput > > ,) -> display :: UnitDisplay < '_ > { display :: UnitDisplay { current_value , upper_bound , throughput : throughput . into () , parent : self , display : display :: What :: ValuesAndUnit , } } # [doc = " Return `self` as trait object implementing `DisplayValue`."] pub fn as_display_value (& self) -> & dyn DisplayValue { match self . kind { Kind :: Label (ref unit) => unit , Kind :: Dynamic (ref unit) => unit . deref () , } } }
};
}
