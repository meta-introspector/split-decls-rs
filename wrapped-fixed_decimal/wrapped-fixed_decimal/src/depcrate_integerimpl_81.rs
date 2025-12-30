// Generated macro for impl_81 (impl)
macro_rules! Depcrate_integerimpl_81 {
() => {
// Module: crate::integer
// Provides: {"impl_81"}
// Dependencies: {}
impl FromStr for FixedInteger { type Err = ParseError ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Self :: try_from_str (s) } }
};
}
