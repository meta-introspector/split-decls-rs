macro_rules! deps {
    () => {
        FullTypeResolver!();
        TyOrConstInferVar!();
        FixupError!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < 'a , 'tcx > FallibleTypeFolder < TyCtxt < 'tcx > > for FullTypeResolver < 'a , 'tcx > { type Error = FixupError ; fn cx (& self) -> TyCtxt < 'tcx > { self . infcx . tcx } fn try_fold_ty (& mut self , t : Ty < 'tcx >) -> Result < Ty < 'tcx > , Self :: Error > { if ! t . has_infer () { Ok (t) } else { let t = self . infcx . shallow_resolve (t) ; match * t . kind () { ty :: Infer (ty :: TyVar (vid)) => { Err (FixupError { unresolved : TyOrConstInferVar :: Ty (vid) }) } ty :: Infer (ty :: IntVar (vid)) => { Err (FixupError { unresolved : TyOrConstInferVar :: TyInt (vid) }) } ty :: Infer (ty :: FloatVar (vid)) => { Err (FixupError { unresolved : TyOrConstInferVar :: TyFloat (vid) }) } ty :: Infer (_) => { bug ! ("Unexpected type in full type resolver: {:?}" , t) ; } _ => t . try_super_fold_with (self) , } } } fn try_fold_region (& mut self , r : ty :: Region < 'tcx >) -> Result < ty :: Region < 'tcx > , Self :: Error > { match r . kind () { ty :: ReVar (_) => Ok (self . infcx . lexical_region_resolutions . borrow () . as_ref () . expect ("region resolution not performed") . resolve_region (self . infcx . tcx , r)) , _ => Ok (r) , } } fn try_fold_const (& mut self , c : ty :: Const < 'tcx >) -> Result < ty :: Const < 'tcx > , Self :: Error > { if ! c . has_infer () { Ok (c) } else { let c = self . infcx . shallow_resolve_const (c) ; match c . kind () { ty :: ConstKind :: Infer (InferConst :: Var (vid)) => { return Err (FixupError { unresolved : super :: TyOrConstInferVar :: Const (vid) }) ; } ty :: ConstKind :: Infer (InferConst :: Fresh (_)) => { bug ! ("Unexpected const in full const resolver: {:?}" , c) ; } _ => { } } c . try_super_fold_with (self) } } }
    };
}

impl_185!();