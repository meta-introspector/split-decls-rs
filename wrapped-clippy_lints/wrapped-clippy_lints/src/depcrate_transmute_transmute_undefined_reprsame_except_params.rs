// Generated macro for same_except_params (function)
macro_rules! Depcrate_transmute_transmute_undefined_reprsame_except_params {
() => {
// Module: crate::transmute::transmute_undefined_repr
// Provides: {"same_except_params"}
// Dependencies: {}
fn same_except_params < 'tcx > (subs1 : GenericArgsRef < 'tcx > , subs2 : GenericArgsRef < 'tcx >) -> bool { for (ty1 , ty2) in subs1 . types () . zip (subs2 . types ()) . filter (| (ty1 , ty2) | ty1 != ty2) { match (ty1 . kind () , ty2 . kind ()) { (ty :: Param (_) , _) | (_ , ty :: Param (_)) => () , (ty :: Adt (adt1 , subs1) , ty :: Adt (adt2 , subs2)) if adt1 == adt2 && same_except_params (subs1 , subs2) => () , _ => return false , } } true }
};
}
