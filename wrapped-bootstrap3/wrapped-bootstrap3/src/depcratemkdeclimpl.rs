// Generated macro for mkdeclimpl (macro)
macro_rules! Depcratemkdeclimpl {
() => {
// Module: crate
// Provides: {"mkdeclimpl"}
// Dependencies: {}
macro_rules ! mkdeclimpl { (impl $ ($ tt : tt) *) => { println ! ("🔧 WRAPPED: mkdeclimpl! macro called") ; impl $ ($ tt) * } ; }
};
}
