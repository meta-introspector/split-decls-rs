// Generated macro for impl_370 (impl)
macro_rules! Depcrate_expand_autodiff_attrsimpl_370 {
() => {
// Module: crate::expand::autodiff_attrs
// Provides: {"impl_370"}
// Dependencies: {}
impl FromStr for DiffActivity { type Err = () ; fn from_str (s : & str) -> Result < DiffActivity , () > { match s { "None" => Ok (DiffActivity :: None) , "Active" => Ok (DiffActivity :: Active) , "ActiveOnly" => Ok (DiffActivity :: ActiveOnly) , "Const" => Ok (DiffActivity :: Const) , "Dual" => Ok (DiffActivity :: Dual) , "Dualv" => Ok (DiffActivity :: Dualv) , "DualOnly" => Ok (DiffActivity :: DualOnly) , "DualvOnly" => Ok (DiffActivity :: DualvOnly) , "Duplicated" => Ok (DiffActivity :: Duplicated) , "DuplicatedOnly" => Ok (DiffActivity :: DuplicatedOnly) , _ => Err (()) , } } }
};
}
