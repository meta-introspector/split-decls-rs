// Generated macro for is_mutable_ty (function)
macro_rules! Depcrate_functions_must_useis_mutable_ty {
() => {
// Module: crate::functions::must_use
// Provides: {"is_mutable_ty"}
// Dependencies: {}
fn is_mutable_ty < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx > , tys : & mut DefIdSet) -> bool { match * ty . kind () { ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Str => false , ty :: Adt (adt , args) => { tys . insert (adt . did ()) && ! ty . is_freeze (cx . tcx , cx . typing_env ()) || matches ! (cx . tcx . get_diagnostic_name (adt . did ()) , Some (sym :: Rc | sym :: Arc)) && args . types () . any (| ty | is_mutable_ty (cx , ty , tys)) } , ty :: Tuple (args) => args . iter () . any (| ty | is_mutable_ty (cx , ty , tys)) , ty :: Array (ty , _) | ty :: Slice (ty) => is_mutable_ty (cx , ty , tys) , ty :: RawPtr (ty , mutbl) | ty :: Ref (_ , ty , mutbl) => mutbl == hir :: Mutability :: Mut || is_mutable_ty (cx , ty , tys) , _ => true , } }
};
}
