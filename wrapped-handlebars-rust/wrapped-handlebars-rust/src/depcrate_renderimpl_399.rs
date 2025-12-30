// Generated macro for impl_399 (impl)
macro_rules! Depcrate_renderimpl_399 {
() => {
// Module: crate::render
// Provides: {"impl_399"}
// Dependencies: {}
impl Evaluable for Template { fn eval < 'reg : 'rc , 'rc > (& 'rc self , registry : & 'reg Registry < 'reg > , ctx : & 'rc Context , rc : & mut RenderContext < 'reg , 'rc > ,) -> Result < () , RenderError > { let iter = self . elements . iter () ; for (idx , t) in iter . enumerate () { t . eval (registry , ctx , rc) . map_err (| mut e | { if e . line_no . is_none () { if let Some (& TemplateMapping (line , col)) = self . mapping . get (idx) { e . line_no = Some (line) ; e . column_no = Some (col) ; } } e . template_name . clone_from (& self . name) ; e }) ? ; } Ok (()) } }
};
}
