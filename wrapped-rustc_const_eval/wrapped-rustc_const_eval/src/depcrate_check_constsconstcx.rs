// Generated macro for ConstCx (struct)
macro_rules! Depcrate_check_constsConstCx {
() => {
// Module: crate::check_consts
// Provides: {"ConstCx"}
// Dependencies: {}
# [doc = " Information about the item currently being const-checked, as well as a reference to the global"] # [doc = " context."] pub struct ConstCx < 'mir , 'tcx > { pub body : & 'mir mir :: Body < 'tcx > , pub tcx : TyCtxt < 'tcx > , pub typing_env : ty :: TypingEnv < 'tcx > , pub const_kind : Option < hir :: ConstContext > , }
};
}
