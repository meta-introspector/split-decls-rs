// Generated macro for deserialize_value_ref_number (macro)
macro_rules! Depcrate_value_dedeserialize_value_ref_number {
() => {
// Module: crate::value::de
// Provides: {"deserialize_value_ref_number"}
// Dependencies: {}
macro_rules ! deserialize_value_ref_number { ($ method : ident) => { # [cfg (not (feature = "arbitrary_precision"))] fn $ method < V > (self , visitor : V) -> Result < V :: Value , Error > where V : Visitor <'de >, { match self { Value :: Number (n) => n . deserialize_any (visitor) , _ => Err (self . invalid_type (& visitor)) , } } # [cfg (feature = "arbitrary_precision")] fn $ method < V > (self , visitor : V) -> Result < V :: Value , Error > where V : Visitor <'de >, { match self { Value :: Number (n) => n .$ method (visitor) , _ => self . deserialize_any (visitor) , } } } ; }
};
}
