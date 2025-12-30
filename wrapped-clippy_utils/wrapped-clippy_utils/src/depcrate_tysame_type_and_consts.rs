// Generated macro for same_type_and_consts (function)
macro_rules! Depcrate_tysame_type_and_consts {
() => {
// Module: crate::ty
// Provides: {"same_type_and_consts"}
// Dependencies: {}
# [doc = " Returns `true` if types `a` and `b` are same types having same `Const` generic args,"] # [doc = " otherwise returns `false`"] pub fn same_type_and_consts < 'tcx > (a : Ty < 'tcx > , b : Ty < 'tcx >) -> bool { match (& a . kind () , & b . kind ()) { (& ty :: Adt (did_a , args_a) , & ty :: Adt (did_b , args_b)) => { if did_a != did_b { return false ; } args_a . iter () . zip (args_b . iter ()) . all (| (arg_a , arg_b) | match (arg_a . kind () , arg_b . kind ()) { (GenericArgKind :: Const (inner_a) , GenericArgKind :: Const (inner_b)) => inner_a == inner_b , (GenericArgKind :: Type (type_a) , GenericArgKind :: Type (type_b)) => { same_type_and_consts (type_a , type_b) } , _ => true , }) } , _ => a == b , } }
};
}
