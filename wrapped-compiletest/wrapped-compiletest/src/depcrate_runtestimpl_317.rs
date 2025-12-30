// Generated macro for impl_317 (impl)
macro_rules! Depcrate_runtestimpl_317 {
() => {
// Module: crate::runtest
// Provides: {"impl_317"}
// Dependencies: {}
impl ProcRes { # [must_use] pub fn format_info (& self) -> String { fn render (name : & str , contents : & str) -> String { let contents = json :: extract_rendered (contents) ; let contents = contents . trim_end () ; if contents . is_empty () { format ! ("{name}: none") } else { format ! ("\
                     --- {name} -------------------------------\n\
                     {contents}\n\
                     ------------------------------------------" ,) } } format ! ("status: {}\ncommand: {}\n{}\n{}\n" , self . status , self . cmdline , render ("stdout" , & self . stdout) , render ("stderr" , & self . stderr) ,) } }
};
}
