// Generated macro for fn_arg_mutability (function)
macro_rules! Depcrate_utilsfn_arg_mutability {
() => {
// Module: crate::utils
// Provides: {"fn_arg_mutability"}
// Dependencies: {}
pub (crate) fn fn_arg_mutability (arg : & FnArg) -> Option < syn :: token :: Mut > { match arg { FnArg :: Typed (syn :: PatType { pat , .. }) => match pat . as_ref () { syn :: Pat :: Ident (syn :: PatIdent { mutability , .. }) => * mutability , _ => None , } , _ => None , } }
};
}
