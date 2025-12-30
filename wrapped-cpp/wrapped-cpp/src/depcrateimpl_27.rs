// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl FromStr for APIStyle { type Err = anyhow :: Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "asymmetric" => Ok (APIStyle :: Asymmetric) , "symmetric" => Ok (APIStyle :: Symmetric) , _ => bail ! ("unrecognized API style: `{}`; expected `asymmetric` or `symmetric`" , s) , } } }
};
}
