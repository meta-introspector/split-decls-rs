// Generated macro for derive_style (macro)
macro_rules! Depcrate_duration_optionsderive_style {
() => {
// Module: crate::duration::options
// Provides: {"derive_style"}
// Dependencies: {}
macro_rules ! derive_style { ($ ($ (# [$ enum_meta : meta]) * pub enum $ enum_name : ident { $ ($ (# [$ variant_meta : meta]) * $ variant : ident) ,* $ (,) ? }) +) => { $ ($ (# [$ enum_meta]) * pub enum $ enum_name { $ ($ (# [$ variant_meta]) * $ variant ,) * } impl From <$ enum_name > for FieldStyle { fn from (style : $ enum_name) -> Self { # [allow (unreachable_patterns)] match style { $ ($ enum_name ::$ variant => FieldStyle ::$ variant ,) * } } } impl TryFrom < FieldStyle > for $ enum_name { type Error = FieldStyle ; fn try_from (style : FieldStyle) -> Result < Self , Self :: Error > { match style { $ (FieldStyle ::$ variant => Ok ($ enum_name ::$ variant) ,) * rest => Err (rest) , } } }) + } ; }
};
}
