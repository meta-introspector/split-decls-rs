// Generated macro for is_call_with_ref_arg (function)
macro_rules! Depcrate_redundant_cloneis_call_with_ref_arg {
() => {
// Module: crate::redundant_clone
// Provides: {"is_call_with_ref_arg"}
// Dependencies: {}
# [doc = " If `kind` is `y = func(x: &T)` where `T: !Copy`, returns `(DefId of func, x, T, y)`."] fn is_call_with_ref_arg < 'tcx > (cx : & LateContext < 'tcx > , mir : & 'tcx mir :: Body < 'tcx > , kind : & 'tcx mir :: TerminatorKind < 'tcx > ,) -> Option < (def_id :: DefId , mir :: Local , Ty < 'tcx > , mir :: Local) > { if let mir :: TerminatorKind :: Call { func , args , destination , .. } = kind && args . len () == 1 && let mir :: Operand :: Move (mir :: Place { local , .. }) = & args [0] . node && let ty :: FnDef (def_id , _) = * func . ty (mir , cx . tcx) . kind () && let (inner_ty , 1 , _) = peel_and_count_ty_refs (args [0] . node . ty (mir , cx . tcx)) && ! is_copy (cx , inner_ty) { Some ((def_id , * local , inner_ty , destination . as_local () ?)) } else { None } }
};
}
