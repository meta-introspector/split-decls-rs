// Generated macro for valid_input_activity (function)
macro_rules! Depcrate_expand_autodiff_attrsvalid_input_activity {
() => {
// Module: crate::expand::autodiff_attrs
// Provides: {"valid_input_activity"}
// Dependencies: {}
pub fn valid_input_activity (mode : DiffMode , activity : DiffActivity) -> bool { use DiffActivity :: * ; return match mode { DiffMode :: Error => false , DiffMode :: Source => false , DiffMode :: Forward => activity . is_dual_or_const () , DiffMode :: Reverse => { matches ! (activity , Active | ActiveOnly | Duplicated | DuplicatedOnly | Const) } } ; }
};
}
