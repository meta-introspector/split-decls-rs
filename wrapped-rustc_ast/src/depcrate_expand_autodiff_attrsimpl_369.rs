// Generated macro for impl_369 (impl)
macro_rules! Depcrate_expand_autodiff_attrsimpl_369 {
() => {
// Module: crate::expand::autodiff_attrs
// Provides: {"impl_369"}
// Dependencies: {}
impl FromStr for DiffMode { type Err = () ; fn from_str (s : & str) -> Result < DiffMode , () > { match s { "Error" => Ok (DiffMode :: Error) , "Source" => Ok (DiffMode :: Source) , "Forward" => Ok (DiffMode :: Forward) , "Reverse" => Ok (DiffMode :: Reverse) , _ => Err (()) , } } }
};
}
