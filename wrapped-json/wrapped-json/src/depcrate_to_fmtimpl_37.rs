// Generated macro for impl_37 (impl)
macro_rules! Depcrate_to_fmtimpl_37 {
() => {
// Module: crate::to_fmt
// Provides: {"impl_37"}
// Dependencies: {}
impl TextHandler { const fn native () -> Self { TextHandler :: Native } const fn number () -> Self { TextHandler :: Number (NumberTextHandler { sign_negative : false , leading_zeroes : 0 , at_start : true , is_nan_or_infinity : false , }) } fn text_fragment (& mut self , v : & str , mut out : impl Write) -> fmt :: Result { match self { TextHandler :: Native => out . write_str (v) , TextHandler :: Number (number) => number . text_fragment (v , out) , } } }
};
}
