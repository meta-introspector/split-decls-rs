// Generated macro for impl_703 (impl)
macro_rules! Depcrate_output_render_linksimpl_703 {
() => {
// Module: crate::output::render::links
// Provides: {"impl_703"}
// Dependencies: {}
# [cfg (unix)] impl f :: Links { pub fn render < C : Colours > (& self , colours : & C , numeric : & NumericLocale) -> TextCell { let style = if self . multiple { colours . multi_link_file () } else { colours . normal () } ; TextCell :: paint (style , numeric . format_int (self . count)) } }
};
}
