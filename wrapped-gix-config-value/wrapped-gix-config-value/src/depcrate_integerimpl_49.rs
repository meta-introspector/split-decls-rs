// Generated macro for impl_49 (impl)
macro_rules! Depcrate_integerimpl_49 {
() => {
// Module: crate::integer
// Provides: {"impl_49"}
// Dependencies: {}
impl FromStr for Suffix { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "k" | "K" => Ok (Self :: Kibi) , "m" | "M" => Ok (Self :: Mebi) , "g" | "G" => Ok (Self :: Gibi) , _ => Err (()) , } } }
};
}
