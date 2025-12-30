// Generated macro for impl_92 (impl)
macro_rules! Depcrate_smart_display_implsimpl_92 {
() => {
// Module: crate::smart_display_impls
// Provides: {"impl_92"}
// Dependencies: {}
impl SmartDisplay for char { type Metadata = () ; fn metadata (& self , _ : FormatterOptions) -> Metadata < '_ , Self > { let mut buf = [0 ; 4] ; let c = self . encode_utf8 (& mut buf) ; Metadata :: new (c . len () , self , ()) } fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { Display :: fmt (self , f) } }
};
}
