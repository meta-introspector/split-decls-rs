// Generated macro for impl_int (macro)
macro_rules! Depcrate_smart_display_implsimpl_int {
() => {
// Module: crate::smart_display_impls
// Provides: {"impl_int"}
// Dependencies: {}
# [doc = " Implement [`SmartDisplay`] for signed integers."] macro_rules ! impl_int { ($ ($ t : ty) *) => { $ (impl SmartDisplay for $ t { type Metadata = () ; fn metadata (& self , f : FormatterOptions) -> Metadata <'_ , Self > { let mut width = if f . sign_plus () || * self < 0 { 1 } else { 0 } ; width += self . unsigned_abs () . checked_ilog10 () . map_or (1 , | n | n as usize + 1) ; Metadata :: new (width , self , ()) } # [inline] fn fmt (& self , f : & mut Formatter <'_ >) -> fmt :: Result { Display :: fmt (self , f) } }) * } ; }
};
}
