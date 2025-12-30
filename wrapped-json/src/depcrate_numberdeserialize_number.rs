// Generated macro for deserialize_number (macro)
macro_rules! Depcrate_numberdeserialize_number {
() => {
// Module: crate::number
// Provides: {"deserialize_number"}
// Dependencies: {}
macro_rules ! deserialize_number { ($ deserialize : ident => $ visit : ident) => { # [cfg (not (feature = "arbitrary_precision"))] fn $ deserialize < V > (self , visitor : V) -> Result < V :: Value , Error > where V : Visitor <'de >, { self . deserialize_any (visitor) } # [cfg (feature = "arbitrary_precision")] fn $ deserialize < V > (self , visitor : V) -> Result < V :: Value , Error > where V : de :: Visitor <'de >, { visitor .$ visit (tri ! (self . n . parse () . map_err (| _ | invalid_number ()))) } } ; }
};
}
