// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl FromStr for ReprEndianness { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "big" => Ok (ReprEndianness :: Big) , "little" => Ok (ReprEndianness :: Little) , _ => Err (()) , } } }
};
}
