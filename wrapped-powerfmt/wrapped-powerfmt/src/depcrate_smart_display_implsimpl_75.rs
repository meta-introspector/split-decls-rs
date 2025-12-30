// Generated macro for impl_75 (impl)
macro_rules! Depcrate_smart_display_implsimpl_75 {
() => {
// Module: crate::smart_display_impls
// Provides: {"impl_75"}
// Dependencies: {}
impl SmartDisplay for bool { type Metadata = () ; # [inline] fn metadata (& self , _ : FormatterOptions) -> Metadata < '_ , Self > { Metadata :: new (if * self { 4 } else { 5 } , self , ()) } # [inline] fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { Display :: fmt (self , f) } }
};
}
