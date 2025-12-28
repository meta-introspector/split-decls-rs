macro_rules! deps {
    () => {
        PrintExtra!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < 'tcx > PrintExtra < 'tcx > { fn with_krate < F , R > (& self , f : F) -> R where F : FnOnce (& ast :: Crate) -> R , { match self { PrintExtra :: AfterParsing { krate , .. } => f (krate) , PrintExtra :: NeedsAstMap { tcx } => f (& tcx . resolver_for_lowering () . borrow () . 1) , } } fn tcx (& self) -> TyCtxt < 'tcx > { match self { PrintExtra :: AfterParsing { .. } => bug ! ("PrintExtra::tcx") , PrintExtra :: NeedsAstMap { tcx } => * tcx , } } }
    };
}

impl_23!()