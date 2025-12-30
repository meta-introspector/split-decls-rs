// Generated macro for c_width_prefix (function)
macro_rules! Depcrate_literalc_width_prefix {
() => {
// Module: crate::literal
// Provides: {"c_width_prefix"}
// Dependencies: {}
fn c_width_prefix (i : & [u8]) -> nom :: IResult < & [u8] , & [u8] > { alt ((tag ("u8") , tag ("u") , tag ("U") , tag ("L"))) (i) }
};
}
