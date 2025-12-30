// Generated macro for impl_688 (impl)
macro_rules! Depcrate_output_render_groupsimpl_688 {
() => {
// Module: crate::output::render::groups
// Provides: {"impl_688"}
// Dependencies: {}
impl Render for Option < f :: Group > { fn render < C : Colours , U : Users + Groups > (self , colours : & C , users : & U , user_format : UserFormat , group_format : GroupFormat , file_user : Option < User > ,) -> TextCell { use uzers :: os :: unix :: GroupExt ; let mut style = colours . not_yours () ; let group = match self { Some (g) => match users . get_group_by_gid (g . 0) { Some (g) => (* g) . clone () , None => return TextCell :: paint (style , g . 0 . to_string ()) , } , None => return TextCell :: blank (colours . no_group ()) , } ; let current_uid = users . get_current_uid () ; if let Some (current_user) = users . get_user_by_uid (current_uid) { if current_user . primary_group_id () == group . gid () || group . members () . iter () . any (| u | u == current_user . name ()) { style = colours . yours () ; } } if group . gid () == 0 && style != colours . yours () { style = colours . root_group () ; } let mut group_name = match user_format { UserFormat :: Name => group . name () . to_string_lossy () . into () , UserFormat :: Numeric => group . gid () . to_string () , } ; if let GroupFormat :: Smart = group_format { if let Some (file_uid) = file_user { if let Some (file_user) = users . get_user_by_uid (file_uid . 0) { if file_user . name () . to_string_lossy () == group . name () . to_string_lossy () { group_name = ":" . to_string () ; } } } } TextCell :: paint (style , group_name) } }
};
}
