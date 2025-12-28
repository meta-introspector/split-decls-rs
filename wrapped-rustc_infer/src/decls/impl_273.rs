macro_rules! deps {
    () => {
        TyOrConstInferVar!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl < 'tcx > TyOrConstInferVar { # [doc = " Tries to extract an inference variable from a type or a constant, returns `None`"] # [doc = " for types other than `ty::Infer(_)` (or `InferTy::Fresh*`) and"] # [doc = " for constants other than `ty::ConstKind::Infer(_)` (or `InferConst::Fresh`)."] pub fn maybe_from_generic_arg (arg : GenericArg < 'tcx >) -> Option < Self > { match arg . kind () { GenericArgKind :: Type (ty) => Self :: maybe_from_ty (ty) , GenericArgKind :: Const (ct) => Self :: maybe_from_const (ct) , GenericArgKind :: Lifetime (_) => None , } } # [doc = " Tries to extract an inference variable from a type or a constant, returns `None`"] # [doc = " for types other than `ty::Infer(_)` (or `InferTy::Fresh*`) and"] # [doc = " for constants other than `ty::ConstKind::Infer(_)` (or `InferConst::Fresh`)."] pub fn maybe_from_term (term : Term < 'tcx >) -> Option < Self > { match term . kind () { TermKind :: Ty (ty) => Self :: maybe_from_ty (ty) , TermKind :: Const (ct) => Self :: maybe_from_const (ct) , } } # [doc = " Tries to extract an inference variable from a type, returns `None`"] # [doc = " for types other than `ty::Infer(_)` (or `InferTy::Fresh*`)."] fn maybe_from_ty (ty : Ty < 'tcx >) -> Option < Self > { match * ty . kind () { ty :: Infer (ty :: TyVar (v)) => Some (TyOrConstInferVar :: Ty (v)) , ty :: Infer (ty :: IntVar (v)) => Some (TyOrConstInferVar :: TyInt (v)) , ty :: Infer (ty :: FloatVar (v)) => Some (TyOrConstInferVar :: TyFloat (v)) , _ => None , } } # [doc = " Tries to extract an inference variable from a constant, returns `None`"] # [doc = " for constants other than `ty::ConstKind::Infer(_)` (or `InferConst::Fresh`)."] fn maybe_from_const (ct : ty :: Const < 'tcx >) -> Option < Self > { match ct . kind () { ty :: ConstKind :: Infer (InferConst :: Var (v)) => Some (TyOrConstInferVar :: Const (v)) , _ => None , } } }
    };
}

impl_273!()