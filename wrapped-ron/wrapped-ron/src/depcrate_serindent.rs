// Generated macro for indent (function)
macro_rules! Depcrate_serindent {
() => {
// Module: crate::ser
// Provides: {"indent"}
// Dependencies: {}
fn indent < W : fmt :: Write > (output : & mut W , config : & PrettyConfig , pretty : & Pretty) -> fmt :: Result { if pretty . indent <= config . depth_limit { for _ in 0 .. pretty . indent { output . write_str (& config . indentor) ? ; } } Ok (()) }
};
}
