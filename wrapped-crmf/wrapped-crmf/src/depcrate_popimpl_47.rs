// Generated macro for impl_47 (impl)
macro_rules! Depcrate_popimpl_47 {
() => {
// Module: crate::pop
// Provides: {"impl_47"}
// Dependencies: {}
impl :: der :: EncodeValue for EncKeyWithIdChoice < '_ > { fn encode_value (& self , encoder : & mut impl :: der :: Writer) -> :: der :: Result < () > { match self { Self :: String (variant) => variant . encode_value (encoder) , Self :: GeneralName (variant) => variant . encode_value (encoder) , } } fn value_len (& self) -> :: der :: Result < :: der :: Length > { match self { Self :: String (variant) => variant . value_len () , Self :: GeneralName (variant) => variant . value_len () , } } }
};
}
