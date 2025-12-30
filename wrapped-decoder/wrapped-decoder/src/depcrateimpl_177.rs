// Generated macro for impl_177 (impl)
macro_rules! Depcrateimpl_177 {
() => {
// Module: crate
// Provides: {"impl_177"}
// Dependencies: {}
impl FromStr for Encoding { type Err = anyhow :: Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "raw" => Ok (Encoding :: Raw) , "rzcobs" => Ok (Encoding :: Rzcobs) , _ => anyhow :: bail ! ("Unknown defmt encoding '{}' specified. This is a bug." , s) , } } }
};
}
