// Generated macro for impl_55 (impl)
macro_rules! Depcrate_aot_shells_fishimpl_55 {
() => {
// Module: crate::aot::shells::fish
// Provides: {"impl_55"}
// Dependencies: {}
impl Generator for Fish { fn file_name (& self , name : & str) -> String { format ! ("{name}.fish") } fn generate (& self , cmd : & Command , buf : & mut dyn Write) { self . try_generate (cmd , buf) . expect ("failed to write completion file") ; } fn try_generate (& self , cmd : & Command , buf : & mut dyn Write) -> Result < () , Error > { let bin_name = cmd . get_bin_name () . expect ("crate::generate should have set the bin_name") ; let name = escape_name (bin_name) ; let mut needs_fn_name = & format ! ("__fish_{name}_needs_command") [..] ; let mut using_fn_name = & format ! ("__fish_{name}_using_subcommand") [..] ; if cmd . has_subcommands () { gen_subcommand_helpers (& name , cmd , buf , needs_fn_name , using_fn_name) ; } else { needs_fn_name = "__fish_use_subcommand" ; using_fn_name = "__fish_seen_subcommand_from" ; } let mut buffer = String :: new () ; gen_fish_inner (bin_name , & [] , cmd , & mut buffer , needs_fn_name , using_fn_name ,) ; write ! (buf , "{buffer}") } }
};
}
