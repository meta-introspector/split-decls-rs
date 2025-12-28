macro_rules! deps {
    () => {
        AdjustMode!();
        ResolvedPatKind!();
        ResolvedPat!();
    };
}

macro_rules! impl_326 {
    () => {
        deps!();
        impl < 'tcx > ResolvedPat < 'tcx > { fn adjust_mode (& self) -> AdjustMode { if let ResolvedPatKind :: Path { res , .. } = self . kind && matches ! (res , Res :: Def (DefKind :: Const | DefKind :: AssocConst , _)) { AdjustMode :: Pass } else { AdjustMode :: peel_until_adt (self . ty . ty_adt_def () . map (| adt | adt . did ())) } } }
    };
}

impl_326!()