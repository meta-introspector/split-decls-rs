// Generated macro for deserialize_primitive (macro)
macro_rules! Depcrate_de_simple_typedeserialize_primitive {
() => {
// Module: crate::de::simple_type
// Provides: {"deserialize_primitive"}
// Dependencies: {}
macro_rules ! deserialize_primitive { ($ method : ident) => { fn $ method < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor <'de >, { let de = AtomicDeserializer { content : self . decode () ?, escaped : self . escaped , } ; de .$ method (visitor) } } ; }
};
}
