// Generated macro for deserialize_num (macro)
macro_rules! Depcrate_de_simple_typedeserialize_num {
() => {
// Module: crate::de::simple_type
// Provides: {"deserialize_num"}
// Dependencies: {}
macro_rules ! deserialize_num { ($ method : ident => $ visit : ident) => { # [inline] fn $ method < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor <'de >, { let text : & str = self . content . as_ref () ; match trim_xml_spaces (text) . parse () { Ok (number) => visitor .$ visit (number) , Err (_) => self . deserialize_str (visitor) , } } } ; }
};
}
