// Generated macro for impl_716 (impl)
macro_rules! Depcrate_output_render_permissionsimpl_716 {
() => {
// Module: crate::output::render::permissions
// Provides: {"impl_716"}
// Dependencies: {}
impl RenderPermissions for Option < f :: Permissions > { fn render < C : Colours > (& self , colours : & C , is_regular_file : bool) -> Vec < ANSIString < 'static > > { match self { Some (p) => { let bit = | bit , chr : & 'static str , style : Style | { if bit { style . paint (chr) } else { colours . dash () . paint ("-") } } ; vec ! [bit (p . user_read , "r" , colours . user_read ()) , bit (p . user_write , "w" , colours . user_write ()) , p . user_execute_bit (colours , is_regular_file) , bit (p . group_read , "r" , colours . group_read ()) , bit (p . group_write , "w" , colours . group_write ()) , p . group_execute_bit (colours) , bit (p . other_read , "r" , colours . other_read ()) , bit (p . other_write , "w" , colours . other_write ()) , p . other_execute_bit (colours) ,] } None => iter :: repeat (colours . dash () . paint ("-")) . take (9) . collect () , } } }
};
}
