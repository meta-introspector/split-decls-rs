// Generated macro for cwrite (function)
macro_rules! Depcratecwrite {
() => {
// Module: crate
// Provides: {"cwrite"}
// Dependencies: {}
# [doc = " The same as `write!()`, but parses color tags."] # [proc_macro] # [cfg (feature = "terminfo")] pub fn cwrite (input : TokenStream) -> TokenStream { get_macro ("write" , input , true) }
};
}
