// Generated macro for impl_750 (impl)
macro_rules! Depcrate_output_render_usersimpl_750 {
() => {
// Module: crate::output::render::users
// Provides: {"impl_750"}
// Dependencies: {}
impl Render for Option < f :: User > { fn render < C : Colours , U : Users > (self , colours : & C , users : & U , format : UserFormat) -> TextCell { # [rustfmt :: skip] let uid = match self { Some (u) => u . 0 , None => return TextCell :: blank (colours . no_user ()) , } ; # [rustfmt :: skip] let user_name = match (format , users . get_user_by_uid (uid)) { (_ , None) => uid . to_string () , (UserFormat :: Numeric , _) => uid . to_string () , (UserFormat :: Name , Some (user)) => user . name () . to_string_lossy () . into () , } ; let style = if users . get_current_uid () == uid { colours . you () } else if uid == 0 { colours . root () } else { colours . other () } ; TextCell :: paint (style , user_name) } }
};
}
