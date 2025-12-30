// Generated macro for mkdeclenum (macro)
macro_rules! Depcratemkdeclenum {
() => {
// Module: crate
// Provides: {"mkdeclenum"}
// Dependencies: {}
macro_rules ! mkdeclenum { (enum $ name : ident $ ($ tt : tt) *) => { println ! ("🔧 WRAPPED: mkdeclenum! macro called for enum: {}" , stringify ! ($ name)) ; enum $ name $ ($ tt) * } ; }
};
}
