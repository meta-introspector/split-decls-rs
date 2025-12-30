// Generated macro for impl_combo_get_field (macro)
macro_rules! Depcrate_fieldsetsimpl_combo_get_field {
() => {
// Module: crate::fieldsets
// Provides: {"impl_combo_get_field"}
// Dependencies: {}
macro_rules ! impl_combo_get_field { ($ type : ident , $ composite : ident , $ enum : ident , $ variant : path) => { impl GetField < CompositeFieldSet > for Combo <$ type , $ variant > { # [inline] fn get_field (& self) -> CompositeFieldSet { CompositeFieldSet ::$ composite (Combo :: new (self . dt () . to_enum () , self . z () . to_enum ())) } } impl Combo <$ type , $ variant > { # [doc = " Convert this specific [`Combo`] into a more general [`Combo`]."] # [doc = " Useful when adding to the field of a [`CompositeFieldSet`]."] # [doc = ""] # [doc = " [`CompositeFieldSet`]: enums::CompositeFieldSet"] pub fn into_enums (self) -> Combo <$ enum , ZoneFieldSet > { Combo :: new (self . dt () . to_enum () , self . z () . to_enum ()) } } } ; }
};
}
