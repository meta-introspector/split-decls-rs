// Generated macro for impl_137 (impl)
macro_rules! Depcrateimpl_137 {
() => {
// Module: crate
// Provides: {"impl_137"}
// Dependencies: {}
impl FromStr for Number { type Err = Infallible ; # [allow (deprecated)] # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (Number :: new_from_str (s)) } }
};
}
