// Generated macro for cprint (function)
macro_rules! Depcratecprint {
() => {
// Module: crate
// Provides: {"cprint"}
// Dependencies: {}
# [doc = " The same as `print!()`, but parses color tags."] # [proc_macro] # [cfg (feature = "terminfo")] pub fn cprint (input : TokenStream) -> TokenStream { get_macro ("print" , input , false) }
};
}
