// Generated macro for impl_zone_combo_helpers (macro)
macro_rules! Depcrate_fieldsetsimpl_zone_combo_helpers {
() => {
// Module: crate::fieldsets
// Provides: {"impl_zone_combo_helpers"}
// Dependencies: {}
macro_rules ! impl_zone_combo_helpers { ($ type : ident , $ composite : ident , $ enum : ident) => { impl $ type { # [inline] # [doc = " Associates this field set with a time zone field set."] pub fn with_zone < Z : ZoneMarkers > (self , zone : Z) -> Combo < Self , Z > { Combo :: new (self , zone) } } impl_combo_get_field ! ($ type , $ composite , $ enum , zone :: SpecificLong) ; impl_combo_get_field ! ($ type , $ composite , $ enum , zone :: SpecificShort) ; impl_combo_get_field ! ($ type , $ composite , $ enum , zone :: LocalizedOffsetLong) ; impl_combo_get_field ! ($ type , $ composite , $ enum , zone :: LocalizedOffsetShort) ; impl_combo_get_field ! ($ type , $ composite , $ enum , zone :: GenericLong) ; impl_combo_get_field ! ($ type , $ composite , $ enum , zone :: GenericShort) ; impl_combo_get_field ! ($ type , $ composite , $ enum , zone :: Location) ; impl_combo_get_field ! ($ type , $ composite , $ enum , zone :: ExemplarCity) ; impl_combo_get_field ! ($ type , $ composite , $ enum , ZoneFieldSet) ; } ; }
};
}
