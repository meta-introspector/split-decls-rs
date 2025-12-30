// Generated macro for impl_379 (impl)
macro_rules! Depcrate_re_unicodeimpl_379 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_379"}
// Dependencies: {}
impl < F > Replacer for F where F : FnMut (& Captures) -> String { fn reg_replace < 'a > (& 'a mut self , caps : & Captures) -> Cow < 'a , str > { (* self) (caps) . into () } }
};
}
