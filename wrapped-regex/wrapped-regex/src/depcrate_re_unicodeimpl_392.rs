// Generated macro for impl_392 (impl)
macro_rules! Depcrate_re_unicodeimpl_392 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_392"}
// Dependencies: {}
impl < 'c , 't > fmt :: Debug for CapturesDebug < 'c , 't > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let slot_to_name : HashMap < usize , & str > = self . 0 . named_groups . iter () . map (| (a , b) | (b , a)) . collect () ; let mut map = f . debug_map () ; for (slot , m) in self . 0 . iter_pos () . enumerate () { let m = m . map (| (s , e) | & self . 0 . text [s .. e]) ; if let Some (ref name) = slot_to_name . get (& slot) { map . entry (& name , & m) ; } else { map . entry (& slot , & m) ; } } map . finish () } }
};
}
