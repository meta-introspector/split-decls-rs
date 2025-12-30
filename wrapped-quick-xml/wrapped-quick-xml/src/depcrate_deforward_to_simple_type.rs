// Generated macro for forward_to_simple_type (macro)
macro_rules! Depcrate_deforward_to_simple_type {
() => {
// Module: crate::de
// Provides: {"forward_to_simple_type"}
// Dependencies: {}
macro_rules ! forward_to_simple_type { ($ deserialize : ident , $ ($ mut : tt) ?) => { # [inline] fn $ deserialize < V > ($ ($ mut) ? self , visitor : V) -> Result < V :: Value , DeError > where V : Visitor <'de >, { SimpleTypeDeserializer :: from_text (self . read_string () ?) .$ deserialize (visitor) } } ; }
};
}
