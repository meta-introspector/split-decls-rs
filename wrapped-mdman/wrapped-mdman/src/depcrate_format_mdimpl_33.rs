// Generated macro for impl_33 (impl)
macro_rules! Depcrate_format_mdimpl_33 {
() => {
// Module: crate::format::md
// Provides: {"impl_33"}
// Dependencies: {}
impl MdFormatter { fn render_html (& self , input : & str) -> Result < String , Error > { let parser = crate :: md_parser (input , None) ; let mut html_output : String = String :: with_capacity (input . len () * 3 / 2) ; pulldown_cmark :: html :: push_html (& mut html_output , parser . map (| (e , _r) | e)) ; Ok (html_output) } }
};
}
