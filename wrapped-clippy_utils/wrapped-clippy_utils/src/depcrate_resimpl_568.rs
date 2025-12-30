// Generated macro for impl_568 (impl)
macro_rules! Depcrate_resimpl_568 {
() => {
// Module: crate::res
// Provides: {"impl_568"}
// Dependencies: {}
impl MaybeDef for AdtDef < '_ > { # [inline] fn opt_def_id (self) -> Option < DefId > { Some (self . did ()) } # [inline] fn opt_def < 'tcx > (self , _ : & impl HasTyCtxt < 'tcx >) -> Option < (DefKind , DefId) > { let did = self . did () ; match self . adt_kind () { AdtKind :: Enum => Some ((DefKind :: Enum , did)) , AdtKind :: Struct => Some ((DefKind :: Struct , did)) , AdtKind :: Union => Some ((DefKind :: Union , did)) , } } }
};
}
