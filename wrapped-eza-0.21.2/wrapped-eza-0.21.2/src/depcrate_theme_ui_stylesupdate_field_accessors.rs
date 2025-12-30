// Generated macro for update_field_accessors (macro)
macro_rules! Depcrate_theme_ui_stylesupdate_field_accessors {
() => {
// Module: crate::theme::ui_styles
// Provides: {"update_field_accessors"}
// Dependencies: {}
macro_rules ! update_field_accessors { ($ struct_name : ident , $ ($ field_name : ident : Option <$ type : ty >) ,*) => { impl $ struct_name { $ (pub fn $ field_name (& mut self) -> & mut $ type { if self .$ field_name . is_none () { self .$ field_name = Some (Default :: default ()) ; } self .$ field_name . as_mut () . unwrap () }) * } } ; }
};
}
