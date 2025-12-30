// Generated macro for impl_41 (impl)
macro_rules! Depcrate_backends_svgimpl_41 {
() => {
// Module: crate::backends::svg
// Provides: {"impl_41"}
// Dependencies: {}
impl SVGWriter { fn grow_window (& mut self , point : Point , size : Point) { self . view_size . x = self . view_size . x . max (point . x + size . x + 5.) ; self . view_size . y = self . view_size . y . max (point . y + size . y + 5.) ; } fn get_or_create_font_style (& mut self , font_size : usize) -> String { if let Option :: Some (x) = self . font_style_map . get (& font_size) { return x . 0 . clone () ; } let class_name = format ! ("a{}" , font_size) ; let class_impl = format ! (".a{} {{ font-size: {}px; font-family: Times, serif; }}" , font_size , font_size) ; let impl_ = (class_name . clone () , class_impl) ; self . font_style_map . insert (font_size , impl_) ; class_name } fn emit_svg_font_styles (& self) -> String { let mut content = String :: new () ; content . push_str ("<style>\n") ; for p in self . font_style_map . iter () { content . push_str (& p . 1 . 1) ; content . push ('\n') ; } content . push_str ("</style>\n") ; for p in self . clip_regions . iter () { content . push_str (p) ; content . push ('\n') ; } content } pub fn finalize (& self) -> String { let mut result = String :: new () ; result . push_str (SVG_HEADER) ; let svg_line = format ! ("<svg width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\
            \" xmlns=\"http://www.w3.org/2000/svg\">\n" , self . view_size . x , self . view_size . y , self . view_size . x , self . view_size . y) ; result . push_str (& svg_line) ; result . push_str ("<rect width=\"100%\" height=\"100%\" fill=\"white\" />") ; result . push_str (SVG_DEFS) ; result . push_str (& self . emit_svg_font_styles ()) ; result . push_str (& self . content) ; result . push_str (SVG_FOOTER) ; result } }
};
}
