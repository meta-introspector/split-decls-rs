// Generated macro for impl_760 (impl)
macro_rules! Depcrate_output_render_octalimpl_760 {
() => {
// Module: crate::output::render::octal
// Provides: {"impl_760"}
// Dependencies: {}
impl Render for Option < f :: OctalPermissions > { fn render (& self , style : Style) -> TextCell { match self { Some (p) => { let perm = & p . permissions ; # [rustfmt :: skip] let octal_sticky = f :: OctalPermissions :: bits_to_octal (perm . setuid , perm . setgid , perm . sticky) ; let octal_owner = f :: OctalPermissions :: bits_to_octal (perm . user_read , perm . user_write , perm . user_execute ,) ; let octal_group = f :: OctalPermissions :: bits_to_octal (perm . group_read , perm . group_write , perm . group_execute ,) ; let octal_other = f :: OctalPermissions :: bits_to_octal (perm . other_read , perm . other_write , perm . other_execute ,) ; TextCell :: paint (style , format ! ("{octal_sticky}{octal_owner}{octal_group}{octal_other}") ,) } None => TextCell :: paint (style , "----" . into ()) , } } }
};
}
