// Generated macro for impl_178 (impl)
macro_rules! Depcrate_infer_lexical_region_resolveimpl_178 {
() => {
// Module: crate::infer::lexical_region_resolve
// Provides: {"impl_178"}
// Dependencies: {}
impl < 'tcx > LexicalRegionResolutions < 'tcx > { fn normalize < T > (& self , tcx : TyCtxt < 'tcx > , value : T) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { fold_regions (tcx , value , | r , _db | self . resolve_region (tcx , r)) } fn value (& self , rid : RegionVid) -> & VarValue < 'tcx > { & self . values [rid] } fn value_mut (& mut self , rid : RegionVid) -> & mut VarValue < 'tcx > { & mut self . values [rid] } pub (crate) fn resolve_region (& self , tcx : TyCtxt < 'tcx > , r : ty :: Region < 'tcx > ,) -> ty :: Region < 'tcx > { let result = match r . kind () { ty :: ReVar (rid) => match self . values [rid] { VarValue :: Empty (_) => r , VarValue :: Value (r) => r , VarValue :: ErrorValue => tcx . lifetimes . re_static , } , _ => r , } ; debug ! ("resolve_region({:?}) = {:?}" , r , result) ; result } }
};
}
