// Generated macro for impl_718 (impl)
macro_rules! Depcrate_output_render_permissionsimpl_718 {
() => {
// Module: crate::output::render::permissions
// Provides: {"impl_718"}
// Dependencies: {}
# [cfg (windows)] impl f :: Attributes { pub fn render < C : Colours + FiletypeColours > (self , colours : & C) -> Vec < ANSIString < 'static > > { let bit = | bit , chr : & 'static str , style : Style | { if bit { style . paint (chr) } else { colours . dash () . paint ("-") } } ; vec ! [bit (self . archive , "a" , colours . normal ()) , bit (self . readonly , "r" , colours . user_read ()) , bit (self . hidden , "h" , colours . special_user_file ()) , bit (self . system , "s" , colours . special_other ()) ,] } pub fn render_type < C : Colours + FiletypeColours > (self , colours : & C) -> ANSIString < 'static > { if self . reparse_point { return colours . pipe () . paint ("l") ; } else if self . directory { return colours . directory () . paint ("d") ; } colours . dash () . paint ("-") } }
};
}
