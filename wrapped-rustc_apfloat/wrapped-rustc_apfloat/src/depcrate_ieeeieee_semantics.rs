// Generated macro for ieee_semantics (macro)
macro_rules! Depcrate_ieeeieee_semantics {
() => {
// Module: crate::ieee
// Provides: {"ieee_semantics"}
// Dependencies: {}
macro_rules ! ieee_semantics { ($ ($ (# [$ meta : meta]) * $ name : ident = $ sem : ident ($ bits : tt : $ exp_bits : tt) $ ({ $ ($ extra : tt) * }) ?) ,* $ (,) ?) => { $ (# [doc = concat ! ("Floating point semantics for [`" , stringify ! ($ name) , "`].")] # [doc = ""] # [doc = " See that type for more details."] pub struct $ sem ; $ (# [$ meta]) * pub type $ name = IeeeFloat <$ sem >; impl Semantics for $ sem { const BITS : usize = $ bits ; const EXP_BITS : usize = $ exp_bits ; $ ($ ($ extra) *) ? }) * } }
};
}
