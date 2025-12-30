// Generated macro for impl_378 (impl)
macro_rules! Depcrate_re_unicodeimpl_378 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_378"}
// Dependencies: {}
impl < 't > Replacer for & 't str { fn reg_replace < 'a > (& 'a mut self , caps : & Captures) -> Cow < 'a , str > { caps . expand (* self) . into () } fn no_expand (& mut self) -> Option < Cow < str > > { match self . find ('$') { Some (_) => None , None => Some ((* self) . into ()) , } } }
};
}
