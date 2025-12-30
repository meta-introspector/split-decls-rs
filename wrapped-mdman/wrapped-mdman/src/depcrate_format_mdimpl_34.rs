// Generated macro for impl_34 (impl)
macro_rules! Depcrate_format_mdimpl_34 {
() => {
// Module: crate::format::md
// Provides: {"impl_34"}
// Dependencies: {}
impl super :: Formatter for MdFormatter { fn render (& self , input : & str) -> Result < String , Error > { Ok (input . replace ("\r\n" , "\n")) } fn render_options_start (& self) -> & 'static str { "<dl>\n" } fn render_options_end (& self) -> & 'static str { "</dl>\n" } fn render_option (& self , params : & [& str] , block : & str , man_name : & str) -> Result < String , Error > { let mut result = String :: new () ; fn unwrap_p (t : & str) -> & str { unwrap (t , "<p>" , "</p>") } for param in params { let rendered = self . render_html (param) ? ; let no_p = unwrap_p (& rendered) ; let first = no_p . split_whitespace () . next () . ok_or_else (| | format_err ! ("did not expect option `{}` to be empty" , param)) ? ; let no_tags = trim_tags (first) ; if no_tags . is_empty () { bail ! ("unexpected empty option with no tags `{}`" , param) ; } let id = format ! ("option-{}-{}" , man_name , no_tags) ; write ! (result , "<dt class=\"option-term\" id=\"{id}\">\
                <a class=\"option-anchor\" href=\"#{id}\">{no_p}</a></dt>\n" ,) ? ; } let rendered_block = self . render_html (block) ? ; write ! (result , "<dd class=\"option-desc\">{rendered_block}</dd>\n\n" ,) ? ; Ok (result) } fn linkify_man_to_md (& self , name : & str , section : u8) -> Result < String , Error > { let s = match self . man_map . get (& (name . to_string () , section)) { Some (link) => format ! ("[{}({})]({})" , name , section , link) , None => format ! ("[{}({})]({}.html)" , name , section , name) , } ; Ok (s) } }
};
}
