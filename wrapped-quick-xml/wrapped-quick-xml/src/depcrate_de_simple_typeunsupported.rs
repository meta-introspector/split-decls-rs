// Generated macro for unsupported (macro)
macro_rules! Depcrate_de_simple_typeunsupported {
() => {
// Module: crate::de::simple_type
// Provides: {"unsupported"}
// Dependencies: {}
macro_rules ! unsupported { ($ deserialize : ident $ (($ ($ type : ty) ,*)) ?) => { # [inline] fn $ deserialize < V : Visitor <'de >> (self , $ ($ (_ : $ type ,) *) ? visitor : V) -> Result < V :: Value , Self :: Error > { self . deserialize_str (visitor) } } ; }
};
}
