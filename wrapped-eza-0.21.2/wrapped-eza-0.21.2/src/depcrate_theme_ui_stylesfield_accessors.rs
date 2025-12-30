// Generated macro for field_accessors (macro)
macro_rules! Depcrate_theme_ui_stylesfield_accessors {
() => {
// Module: crate::theme::ui_styles
// Provides: {"field_accessors"}
// Dependencies: {}
macro_rules ! field_accessors { ($ struct_name : ident , $ ($ field_name : ident : Option <$ type : ty >) ,*) => { impl $ struct_name { $ (# [allow (clippy :: wrong_self_convention , clippy :: new_ret_no_self)] pub fn $ field_name (& self) -> $ type { self .$ field_name . unwrap_or_default () }) * } } ; }
};
}
