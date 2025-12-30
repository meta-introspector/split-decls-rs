// Generated macro for implements_trait_with_env (function)
macro_rules! Depcrate_tyimplements_trait_with_env {
() => {
// Module: crate::ty
// Provides: {"implements_trait_with_env"}
// Dependencies: {}
# [doc = " Same as `implements_trait` but allows using a `ParamEnv` different from the lint context."] # [doc = ""] # [doc = " The `callee_id` argument is used to determine whether this is a function call in a `const fn`"] # [doc = " environment, used for checking const traits."] pub fn implements_trait_with_env < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , ty : Ty < 'tcx > , trait_id : DefId , callee_id : Option < DefId > , args : & [GenericArg < 'tcx >] ,) -> bool { implements_trait_with_env_from_iter (tcx , typing_env , ty , trait_id , callee_id , args . iter () . map (| & x | Some (x))) }
};
}
