// Generated macro for position (function)
macro_rules! Depcrate_legacyposition {
() => {
// Module: crate::legacy
// Provides: {"position"}
// Dependencies: {}
fn position (keys : & [Address] , key : & Address) -> u8 { keys . iter () . position (| k | k == key) . unwrap () as u8 }
};
}
