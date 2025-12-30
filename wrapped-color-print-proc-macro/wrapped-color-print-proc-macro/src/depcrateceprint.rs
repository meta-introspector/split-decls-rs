// Generated macro for ceprint (function)
macro_rules! Depcrateceprint {
() => {
// Module: crate
// Provides: {"ceprint"}
// Dependencies: {}
# [doc = " The same as `eprint!()`, but parses color tags."] # [proc_macro] # [cfg (feature = "terminfo")] pub fn ceprint (input : TokenStream) -> TokenStream { get_macro ("eprint" , input , false) }
};
}
