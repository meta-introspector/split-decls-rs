// Generated macro for valid_ret_activity (function)
macro_rules! Depcrate_expand_autodiff_attrsvalid_ret_activity {
() => {
// Module: crate::expand::autodiff_attrs
// Provides: {"valid_ret_activity"}
// Dependencies: {}
# [doc = " Active(Only) is valid in reverse-mode AD for scalar float returns (f16/f32/...)."] # [doc = " Dual(Only) is valid in forward-mode AD for scalar float returns (f16/f32/...)."] # [doc = " Const is valid for all cases and means that we don't compute derivatives wrt. this output."] # [doc = " That usually means we have a &mut or *mut T output and compute derivatives wrt. that arg,"] # [doc = " but this is too complex to verify here. Also it's just a logic error if users get this wrong."] pub fn valid_ret_activity (mode : DiffMode , activity : DiffActivity) -> bool { if activity == DiffActivity :: None { return true ; } match mode { DiffMode :: Error => false , DiffMode :: Source => false , DiffMode :: Forward => activity . is_dual_or_const () , DiffMode :: Reverse => { activity == DiffActivity :: Const || activity == DiffActivity :: Active || activity == DiffActivity :: ActiveOnly } } }
};
}
