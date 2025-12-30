// Generated macro for impl_398 (impl)
macro_rules! Depcrate_renderimpl_398 {
() => {
// Module: crate::render
// Provides: {"impl_398"}
// Dependencies: {}
impl Renderable for Template { fn render < 'reg : 'rc , 'rc > (& 'rc self , registry : & 'reg Registry < 'reg > , ctx : & 'rc Context , rc : & mut RenderContext < 'reg , 'rc > , out : & mut dyn Output ,) -> Result < () , RenderError > { rc . set_current_template_name (self . name . as_ref ()) ; let iter = self . elements . iter () ; for (idx , t) in iter . enumerate () { t . render (registry , ctx , rc , out) . map_err (| mut e | { if e . line_no . is_none () { if let Some (& TemplateMapping (line , col)) = self . mapping . get (idx) { e . line_no = Some (line) ; e . column_no = Some (col) ; } } if e . template_name . is_none () { e . template_name . clone_from (& self . name) ; } e }) ? ; } Ok (()) } }
};
}
