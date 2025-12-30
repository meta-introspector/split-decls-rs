// Generated macro for east_asian_traditional (module)
macro_rules! Depcrate_caleast_asian_traditional {
() => {
// Module: crate::cal
// Provides: {"east_asian_traditional"}
// Dependencies: {}
# [doc = " Customizations for the [`EastAsianTraditional`](east_asian_traditional::EastAsianTraditional) calendar."] pub mod east_asian_traditional { pub use super :: east_asian_traditional_internal :: { China , EastAsianTraditional , Korea } ; # [cfg (feature = "unstable")] pub use super :: east_asian_traditional_internal :: { EastAsianTraditionalYear , Rules } ; }
};
}
