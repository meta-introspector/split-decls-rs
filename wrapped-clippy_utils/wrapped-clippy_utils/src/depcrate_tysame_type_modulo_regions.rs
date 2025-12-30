// Generated macro for same_type_modulo_regions (function)
macro_rules! Depcrate_tysame_type_modulo_regions {
() => {
// Module: crate::ty
// Provides: {"same_type_modulo_regions"}
// Dependencies: {}
# [doc = " Checks whether `a` and `b` are same types having same `Const` generic args, but ignores"] # [doc = " lifetimes."] # [doc = ""] # [doc = " For example, the function would return `true` for"] # [doc = " - `u32` and `u32`"] # [doc = " - `[u8; N]` and `[u8; M]`, if `N=M`"] # [doc = " - `Option<T>` and `Option<U>`, if `same_type_modulo_regions(T, U)` holds"] # [doc = " - `&'a str` and `&'b str`"] # [doc = ""] # [doc = " and `false` for:"] # [doc = " - `Result<u32, String>` and `Result<usize, String>`"] pub fn same_type_modulo_regions < 'tcx > (a : Ty < 'tcx > , b : Ty < 'tcx >) -> bool { match (& a . kind () , & b . kind ()) { (& ty :: Adt (did_a , args_a) , & ty :: Adt (did_b , args_b)) => { if did_a != did_b { return false ; } iter :: zip (* args_a , * args_b) . all (| (arg_a , arg_b) | match (arg_a . kind () , arg_b . kind ()) { (GenericArgKind :: Const (inner_a) , GenericArgKind :: Const (inner_b)) => inner_a == inner_b , (GenericArgKind :: Type (type_a) , GenericArgKind :: Type (type_b)) => { same_type_modulo_regions (type_a , type_b) } , _ => true , }) } , _ => a == b , } }
};
}
