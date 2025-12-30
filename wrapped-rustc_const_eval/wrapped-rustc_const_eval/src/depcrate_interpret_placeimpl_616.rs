// Generated macro for impl_616 (impl)
macro_rules! Depcrate_interpret_placeimpl_616 {
() => {
// Module: crate::interpret::place
// Provides: {"impl_616"}
// Dependencies: {}
impl < Prov : Provenance > MemPlace < Prov > { # [doc = " Adjust the provenance of the main pointer (metadata is unaffected)."] fn map_provenance (self , f : impl FnOnce (Prov) -> Prov) -> Self { MemPlace { ptr : self . ptr . map_provenance (| p | p . map (f)) , .. self } } # [doc = " Turn a mplace into a (thin or wide) pointer, as a reference, pointing to the same space."] # [inline] fn to_ref (self , cx : & impl HasDataLayout) -> Immediate < Prov > { Immediate :: new_pointer_with_meta (self . ptr , self . meta , cx) } # [inline] fn offset_with_meta_ < 'tcx , M : Machine < 'tcx , Provenance = Prov > > (self , offset : Size , mode : OffsetMode , meta : MemPlaceMeta < Prov > , ecx : & InterpCx < 'tcx , M > ,) -> InterpResult < 'tcx , Self > { debug_assert ! (! meta . has_meta () || self . meta . has_meta () , "cannot use `offset_with_meta` to add metadata to a place") ; let ptr = match mode { OffsetMode :: Inbounds => { ecx . ptr_offset_inbounds (self . ptr , offset . bytes () . try_into () . unwrap ()) ? } OffsetMode :: Wrapping => self . ptr . wrapping_offset (offset , ecx) , } ; interp_ok (MemPlace { ptr , meta , misaligned : self . misaligned }) } }
};
}
