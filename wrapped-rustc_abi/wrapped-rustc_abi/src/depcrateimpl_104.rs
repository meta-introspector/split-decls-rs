// Generated macro for impl_104 (impl)
macro_rules! Depcrateimpl_104 {
() => {
// Module: crate
// Provides: {"impl_104"}
// Dependencies: {}
impl FromStr for Endian { type Err = String ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "little" => Ok (Self :: Little) , "big" => Ok (Self :: Big) , _ => Err (format ! (r#"unknown endian: "{s}""#)) , } } }
};
}
