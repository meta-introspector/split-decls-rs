// Generated macro for Output (trait)
macro_rules! Depcrate_outputOutput {
() => {
// Module: crate::output
// Provides: {"Output"}
// Dependencies: {}
# [doc = " The Output API."] # [doc = ""] # [doc = " Handlebars uses this trait to define rendered output."] pub trait Output { fn write (& mut self , seg : & str) -> Result < () , IOError > ; # [doc = " Designed to be used with `write!` macro."] # [doc = " for backward compatibility and to avoid breakage the default implementation"] # [doc = " uses `format!` this may be not what you want."] fn write_fmt (& mut self , args : std :: fmt :: Arguments < '_ >) -> Result < () , IOError > { if let Some (content) = args . as_str () { self . write (content) } else { self . write (& std :: fmt :: format (args)) } } }
};
}
