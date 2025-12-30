// Generated macro for deserialize_number (macro)
macro_rules! Depcrate_dedeserialize_number {
() => {
// Module: crate::de
// Provides: {"deserialize_number"}
// Dependencies: {}
macro_rules ! deserialize_number { ($ method : ident) => { deserialize_number ! ($ method , deserialize_number) ; } ; ($ method : ident , $ using : ident) => { fn $ method < V > (self , visitor : V) -> Result < V :: Value > where V : de :: Visitor <'de >, { self .$ using (visitor) } } ; }
};
}
