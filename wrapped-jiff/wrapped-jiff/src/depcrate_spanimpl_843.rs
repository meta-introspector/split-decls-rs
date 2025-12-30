// Generated macro for impl_843 (impl)
macro_rules! Depcrate_spanimpl_843 {
() => {
// Module: crate::span
// Provides: {"impl_843"}
// Dependencies: {}
impl core :: fmt :: Debug for UnitSet { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{{") ? ; let mut units = * self ; let mut i = 0 ; while let Some (unit) = units . largest_unit () { if i > 0 { write ! (f , ", ") ? ; } i += 1 ; write ! (f , "{}" , unit . compact ()) ? ; units = units . set (unit , false) ; } if i == 0 { write ! (f , "∅") ? ; } write ! (f , "}}") } }
};
}
