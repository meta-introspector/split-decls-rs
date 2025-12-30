// Generated macro for impl_107 (impl)
macro_rules! Depcrateimpl_107 {
() => {
// Module: crate
// Provides: {"impl_107"}
// Dependencies: {}
impl FromStr for Endian { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "little" => Ok (Self :: Little) , "big" => Ok (Self :: Big) , _ => Err (format ! (r#"unknown endian: "{s}""#)) , } } }
};
}
