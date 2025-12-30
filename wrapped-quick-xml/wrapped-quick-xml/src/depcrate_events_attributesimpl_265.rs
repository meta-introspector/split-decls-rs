// Generated macro for impl_265 (impl)
macro_rules! Depcrate_events_attributesimpl_265 {
() => {
// Module: crate::events::attributes
// Provides: {"impl_265"}
// Dependencies: {}
impl < T : AsRef < [u8] > > Debug for Attr < T > { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { match self { Attr :: DoubleQ (key , value) => f . debug_tuple ("Attr::DoubleQ") . field (& Bytes (key . as_ref ())) . field (& Bytes (value . as_ref ())) . finish () , Attr :: SingleQ (key , value) => f . debug_tuple ("Attr::SingleQ") . field (& Bytes (key . as_ref ())) . field (& Bytes (value . as_ref ())) . finish () , Attr :: Empty (key) => f . debug_tuple ("Attr::Empty") . field (& Bytes (key . as_ref ())) . finish () , Attr :: Unquoted (key , value) => f . debug_tuple ("Attr::Unquoted") . field (& Bytes (key . as_ref ())) . field (& Bytes (value . as_ref ())) . finish () , } } }
};
}
