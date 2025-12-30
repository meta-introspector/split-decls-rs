// Generated macro for GetSymbolsFn (type)
macro_rules! DepcrateGetSymbolsFn {
() => {
// Module: crate
// Provides: {"GetSymbolsFn"}
// Dependencies: {}
pub type GetSymbolsFn = fn (buf : & [u8] , f : & mut dyn FnMut (& [u8]) -> std :: io :: Result < () >) -> std :: io :: Result < bool > ;
};
}
