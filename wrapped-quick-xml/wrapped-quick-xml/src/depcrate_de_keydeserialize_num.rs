// Generated macro for deserialize_num (macro)
macro_rules! Depcrate_de_keydeserialize_num {
() => {
// Module: crate::de::key
// Provides: {"deserialize_num"}
// Dependencies: {}
macro_rules ! deserialize_num { ($ method : ident , $ visit : ident) => { fn $ method < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor <'de >, { match self . name . parse () { Ok (number) => visitor .$ visit (number) , Err (_) => self . name . deserialize_str (visitor) , } } } ; }
};
}
