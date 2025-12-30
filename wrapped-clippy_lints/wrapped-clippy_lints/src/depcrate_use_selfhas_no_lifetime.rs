// Generated macro for has_no_lifetime (function)
macro_rules! Depcrate_use_selfhas_no_lifetime {
() => {
// Module: crate::use_self
// Provides: {"has_no_lifetime"}
// Dependencies: {}
# [doc = " Returns `true` if `ty` has no lifetime parameter, otherwise returns `false`."] fn has_no_lifetime (ty : MiddleTy < '_ >) -> bool { use rustc_middle :: ty :: { Adt , GenericArgKind } ; match ty . kind () { & Adt (_ , args) => ! args . iter () . any (| arg | matches ! (arg . kind () , GenericArgKind :: Lifetime (..))) , _ => true , } }
};
}
