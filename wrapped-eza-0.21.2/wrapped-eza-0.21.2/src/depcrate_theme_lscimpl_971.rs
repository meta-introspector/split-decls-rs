// Generated macro for impl_971 (impl)
macro_rules! Depcrate_theme_lscimpl_971 {
() => {
// Module: crate::theme::lsc
// Provides: {"impl_971"}
// Dependencies: {}
impl < 'var > LSColors < 'var > { pub fn each_pair < C > (& mut self , mut callback : C) where C : FnMut (Pair < 'var >) , { for next in self . 0 . split (':') { let bits = next . split ('=') . take (3) . collect :: < Vec < _ > > () ; if bits . len () == 2 && ! bits [0] . is_empty () && ! bits [1] . is_empty () { callback (Pair { key : bits [0] , value : bits [1] , }) ; } } } }
};
}
