// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
# [cfg (feature = "std")] impl :: std :: error :: Error for FromHexError { fn description (& self) -> & str { match * self { InvalidHexCharacter (_ , _) => "invalid character" , InvalidHexLength => "invalid length" , } } }
};
}
