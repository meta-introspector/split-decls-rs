// Generated macro for impl_uint (macro)
macro_rules! Depcrate_smart_display_implsimpl_uint {
() => {
// Module: crate::smart_display_impls
// Provides: {"impl_uint"}
// Dependencies: {}
# [doc = " Implement [`SmartDisplay`] for unsigned integers."] macro_rules ! impl_uint { ($ ($ t : ty) *) => { $ (impl SmartDisplay for $ t { type Metadata = () ; fn metadata (& self , f : FormatterOptions) -> Metadata <'_ , Self > { let mut width = self . checked_ilog10 () . map_or (1 , | n | n as usize + 1) ; if f . sign_plus () || f . sign_minus () { width += 1 ; } Metadata :: new (width , self , ()) } # [inline] fn fmt (& self , f : & mut Formatter <'_ >) -> fmt :: Result { Display :: fmt (self , f) } }) * } ; }
};
}
