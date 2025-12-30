// Generated macro for mkdeclmod (macro)
macro_rules! Depcratemkdeclmod {
() => {
// Module: crate
// Provides: {"mkdeclmod"}
// Dependencies: {}
macro_rules ! mkdeclmod { (mod $ name : ident $ ($ tt : tt) *) => { println ! ("🔧 WRAPPED: mkdeclmod! macro called for module: {}" , stringify ! ($ name)) ; mod $ name $ ($ tt) * } ; }
};
}
