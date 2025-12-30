// Generated macro for same_lifetimes (function)
macro_rules! Depcrate_use_selfsame_lifetimes {
() => {
// Module: crate::use_self
// Provides: {"same_lifetimes"}
// Dependencies: {}
# [doc = " Checks whether types `a` and `b` have the same lifetime parameters."] # [doc = ""] # [doc = " This function does not check that types `a` and `b` are the same types."] fn same_lifetimes < 'tcx > (a : MiddleTy < 'tcx > , b : MiddleTy < 'tcx >) -> bool { use rustc_middle :: ty :: { Adt , GenericArgKind } ; match (a . kind () , b . kind ()) { (Adt (_ , args_a) , Adt (_ , args_b)) => { iter :: zip (* args_a , * args_b) . all (| (arg_a , arg_b) | match (arg_a . kind () , arg_b . kind ()) { (GenericArgKind :: Lifetime (inner_a) , GenericArgKind :: Lifetime (inner_b)) => inner_a == inner_b , (GenericArgKind :: Type (type_a) , GenericArgKind :: Type (type_b)) => same_lifetimes (type_a , type_b) , _ => true , }) } , _ => a == b , } }
};
}
