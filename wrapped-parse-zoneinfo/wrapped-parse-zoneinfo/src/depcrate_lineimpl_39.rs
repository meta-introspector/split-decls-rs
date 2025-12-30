// Generated macro for impl_39 (impl)
macro_rules! Depcrate_lineimpl_39 {
() => {
// Module: crate::line
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'a > Link < 'a > { fn from_str (input : & 'a str) -> Result < Self , Error > { let mut iter = input . split_ascii_whitespace () ; if iter . next () != Some ("Link") { return Err (Error :: NotParsedAsLinkLine) ; } Ok (Link { existing : iter . next () . ok_or (Error :: NotParsedAsLinkLine) ? , new : iter . next () . ok_or (Error :: NotParsedAsLinkLine) ? , }) } }
};
}
