// Generated macro for gen_body (function)
macro_rules! Depcrategen_body {
() => {
// Module: crate
// Provides: {"gen_body"}
// Dependencies: {}
fn gen_body (block : & TokenTree , settings : & Settings) -> proc_macro2 :: TokenStream { let is_proc_macro_hack = settings . is_set (ProcMacroHack) ; let closure = if settings . is_set (AssertUnwindSafe) { quote ! (:: std :: panic :: AssertUnwindSafe (|| # block)) } else { quote ! (|| # block) } ; quote ! (:: proc_macro_error2 :: entry_point (# closure , # is_proc_macro_hack)) }
};
}
