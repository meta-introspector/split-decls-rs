// Generated macro for impl_927 (impl)
macro_rules! Depcrate_patimpl_927 {
() => {
// Module: crate::pat
// Provides: {"impl_927"}
// Dependencies: {}
impl < 'tcx > ResolvedPat < 'tcx > { fn adjust_mode (& self) -> AdjustMode { if let ResolvedPatKind :: Path { res , .. } = self . kind && matches ! (res , Res :: Def (DefKind :: Const | DefKind :: AssocConst , _)) { AdjustMode :: Pass } else { AdjustMode :: peel_until_adt (self . ty . ty_adt_def () . map (| adt | adt . did ())) } } }
};
}
