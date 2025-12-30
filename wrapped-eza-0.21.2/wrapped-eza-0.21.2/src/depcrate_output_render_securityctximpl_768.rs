// Generated macro for impl_768 (impl)
macro_rules! Depcrate_output_render_securityctximpl_768 {
() => {
// Module: crate::output::render::securityctx
// Provides: {"impl_768"}
// Dependencies: {}
impl f :: SecurityContext < '_ > { pub fn render < C : Colours > (& self , colours : & C) -> TextCell { match & self . context { f :: SecurityContextType :: None => TextCell :: paint_str (colours . none () , "?") , f :: SecurityContextType :: SELinux (context) => { let mut chars = Vec :: with_capacity (7) ; for (i , part) in context . split (':') . enumerate () { let partcolour = match i { 0 => colours . selinux_user () , 1 => colours . selinux_role () , 2 => colours . selinux_type () , _ => colours . selinux_range () , } ; if i > 0 { chars . push (colours . selinux_colon () . paint (":")) ; } chars . push (partcolour . paint (String :: from (part))) ; } TextCell { contents : chars . into () , width : DisplayWidth :: from (context . len ()) , } } } } }
};
}
