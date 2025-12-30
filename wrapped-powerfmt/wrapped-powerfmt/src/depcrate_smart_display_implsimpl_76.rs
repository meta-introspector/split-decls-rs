// Generated macro for impl_76 (impl)
macro_rules! Depcrate_smart_display_implsimpl_76 {
() => {
// Module: crate::smart_display_impls
// Provides: {"impl_76"}
// Dependencies: {}
impl SmartDisplay for str { type Metadata = () ; # [inline] fn metadata (& self , f : FormatterOptions) -> Metadata < '_ , Self > { Metadata :: new (match f . precision () { Some (max_len) => min (self . len () , max_len) , None => self . len () , } , self , () ,) } # [inline] fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { Display :: fmt (self , f) } }
};
}
