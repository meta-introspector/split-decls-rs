// Generated macro for impl_20 (impl)
macro_rules! Depcrate_format_manimpl_20 {
() => {
// Module: crate::format::man
// Provides: {"impl_20"}
// Dependencies: {}
impl super :: Formatter for ManFormatter { fn render (& self , input : & str) -> Result < String , Error > { ManRenderer :: render (input , self . url . clone ()) } fn render_options_start (& self) -> & 'static str { "<![CDATA[" } fn render_options_end (& self) -> & 'static str { "]]>" } fn render_option (& self , params : & [& str] , block : & str , _man_name : & str ,) -> Result < String , Error > { let rendered_options = params . iter () . map (| param | { let r = self . render (param) ? ; Ok (r . trim () . trim_start_matches (".sp") . to_string ()) }) . collect :: < Result < Vec < _ > , Error > > () ? ; let rendered_block = self . render (block) ? ; let rendered_block = rendered_block . trim () . trim_start_matches (".sp") . trim () ; Ok (format ! ("\n.sp\n{}\n.RS 4\n{}\n.RE\n" , rendered_options . join (", ") , rendered_block)) } fn linkify_man_to_md (& self , name : & str , section : u8) -> Result < String , Error > { Ok (format ! ("`{}`({})" , name , section)) } }
};
}
