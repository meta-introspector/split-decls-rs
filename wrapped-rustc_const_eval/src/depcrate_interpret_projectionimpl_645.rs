// Generated macro for impl_645 (impl)
macro_rules! Depcrate_interpret_projectionimpl_645 {
() => {
// Module: crate::interpret::projection
// Provides: {"impl_645"}
// Dependencies: {}
impl < 'a , 'tcx , Prov : Provenance , P : Projectable < 'tcx , Prov > > ArrayIterator < 'a , 'tcx , Prov , P > { # [doc = " Should be the same `ecx` on each call, and match the one used to create the iterator."] pub fn next < M : Machine < 'tcx , Provenance = Prov > > (& mut self , ecx : & InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , Option < (u64 , P) > > { let Some (idx) = self . range . next () else { return interp_ok (None) } ; interp_ok (Some ((idx , self . base . offset_with_meta (self . stride * idx , OffsetMode :: Wrapping , MemPlaceMeta :: None , self . field_layout , ecx ,) ? ,))) } }
};
}
