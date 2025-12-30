// Generated macro for cwriteln (function)
macro_rules! Depcratecwriteln {
() => {
// Module: crate
// Provides: {"cwriteln"}
// Dependencies: {}
# [doc = " The same as `writeln!()`, but parses color tags."] # [proc_macro] # [cfg (feature = "terminfo")] pub fn cwriteln (input : TokenStream) -> TokenStream { get_macro ("writeln" , input , true) }
};
}
