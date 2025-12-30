// Generated macro for impl_163 (impl)
macro_rules! Depcrate_font_descriptorimpl_163 {
() => {
// Module: crate::font_descriptor
// Provides: {"impl_163"}
// Dependencies: {}
impl TraitAccessorPrivate for CTFontTraits { fn extract_number_for_key (& self , key : CFStringRef) -> CFNumber { let cftype = self . get (key) ; let number = cftype . downcast :: < CFNumber > () ; match number { Some (number) => number , None => { let value_as_bool = bool :: from (cftype . downcast :: < CFBoolean > () . expect ("Should be able to convert value into CFBoolean") ,) ; CFNumber :: from (value_as_bool as i32) } } } }
};
}
