// Generated macro for impl_27 (impl)
macro_rules! Depcrate_doubleimpl_27 {
() => {
// Module: crate::double
// Provides: {"impl_27"}
// Dependencies: {}
impl FromStr for DoublePlaceholderKey { type Err = Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "0" => Ok (Self :: Place0) , "1" => Ok (Self :: Place1) , _ => Err (Error :: InvalidPlaceholder) , } } }
};
}
