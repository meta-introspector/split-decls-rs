// Generated macro for impl_21 (impl)
macro_rules! Depcrate_displayimpl_21 {
() => {
// Module: crate::display
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a , 'e , E : Engine > Display for Base64Display < 'a , 'e , E > { fn fmt (& self , formatter : & mut Formatter) -> Result < () , fmt :: Error > { let mut sink = FormatterSink { f : formatter } ; self . chunked_encoder . encode (self . bytes , & mut sink) } }
};
}
