// Generated macro for cprintln (function)
macro_rules! Depcratecprintln {
() => {
// Module: crate
// Provides: {"cprintln"}
// Dependencies: {}
# [doc = " The same as `println!()`, but parses color tags."] # [proc_macro] # [cfg (feature = "terminfo")] pub fn cprintln (input : TokenStream) -> TokenStream { get_macro ("println" , input , false) }
};
}
