// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl Generator for Nushell { fn file_name (& self , name : & str) -> String { format ! ("{name}.nu") } fn generate (& self , cmd : & Command , buf : & mut dyn std :: io :: Write) { self . try_generate (cmd , buf) . expect ("failed to write completion file") ; } fn try_generate (& self , cmd : & Command , buf : & mut dyn std :: io :: Write ,) -> Result < () , std :: io :: Error > { let mut completions = String :: new () ; completions . push_str ("module completions {\n\n") ; generate_completion (& mut completions , cmd , false) ; for sub in cmd . get_subcommands () { generate_completion (& mut completions , sub , true) ; } completions . push_str ("}\n\n") ; completions . push_str ("export use completions *\n") ; buf . write_all (completions . as_bytes ()) } }
};
}
