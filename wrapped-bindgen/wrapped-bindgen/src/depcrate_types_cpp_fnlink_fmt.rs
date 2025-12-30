// Generated macro for link_fmt (function)
macro_rules! Depcrate_types_cpp_fnlink_fmt {
() => {
// Module: crate::types::cpp_fn
// Provides: {"link_fmt"}
// Dependencies: {}
fn link_fmt (tokens : TokenStream) -> TokenStream { let mut tokens = tokens . 0 . replacen (" ! (  " , "!(" , 1) ; tokens = tokens . replacen (" ( " , "(" , 1) ; tokens = tokens . replace (" , " , ", ") ; tokens = tokens . replace (" )" , ")") ; tokens . into () }
};
}
