// Generated macro for impl_46 (impl)
macro_rules! Depcrate_format_textimpl_46 {
() => {
// Module: crate::format::text
// Provides: {"impl_46"}
// Dependencies: {}
impl super :: Formatter for TextFormatter { fn render (& self , input : & str) -> Result < String , Error > { TextRenderer :: render (input , self . url . clone () , 0) } fn render_options_start (& self) -> & 'static str { "<![CDATA[\n" } fn render_options_end (& self) -> & 'static str { "]]>\n" } fn render_option (& self , params : & [& str] , block : & str , _man_name : & str ,) -> Result < String , Error > { let rendered_options = params . iter () . map (| param | TextRenderer :: render (param , self . url . clone () , 0)) . collect :: < Result < Vec < _ > , Error > > () ? ; let trimmed : Vec < _ > = rendered_options . iter () . map (| o | o . trim ()) . collect () ; Ok (format ! ("<dt>{}</dt>\n<dd>\n{}</dd>\n<br>\n" , trimmed . join (", ") , block)) } fn linkify_man_to_md (& self , name : & str , section : u8) -> Result < String , Error > { Ok (format ! ("`{}`({})" , name , section)) } }
};
}
