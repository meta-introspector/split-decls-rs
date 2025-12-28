macro_rules! deps {
    () => {
        InferenceLiteralEraser!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for InferenceLiteralEraser < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_ty (& mut self , ty : Ty < 'tcx >) -> Ty < 'tcx > { match ty . kind () { ty :: Infer (ty :: IntVar (_) | ty :: FreshIntTy (_)) => self . tcx . types . i32 , ty :: Infer (ty :: FloatVar (_) | ty :: FreshFloatTy (_)) => self . tcx . types . f64 , _ => ty . super_fold_with (self) , } } }
    };
}

impl_275!()