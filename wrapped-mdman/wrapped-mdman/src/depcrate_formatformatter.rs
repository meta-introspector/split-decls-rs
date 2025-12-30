// Generated macro for Formatter (trait)
macro_rules! Depcrate_formatFormatter {
() => {
// Module: crate::format
// Provides: {"Formatter"}
// Dependencies: {}
pub trait Formatter { # [doc = " Renders the given markdown to the formatter's output."] fn render (& self , input : & str) -> Result < String , Error > ; # [doc = " Renders the start of a block of options (triggered by `{{#options}}`)."] fn render_options_start (& self) -> & 'static str ; # [doc = " Renders the end of a block of options (triggered by `{{/options}}`)."] fn render_options_end (& self) -> & 'static str ; # [doc = " Renders an option (triggered by `{{#option}}`)."] fn render_option (& self , params : & [& str] , block : & str , man_name : & str) -> Result < String , Error > ; # [doc = " Converts a man page reference into markdown that is appropriate for this format."] # [doc = ""] # [doc = " Triggered by `{{man name section}}`."] fn linkify_man_to_md (& self , name : & str , section : u8) -> Result < String , Error > ; }
};
}
