// Generated macro for mkdeclstruct (macro)
macro_rules! Depcratemkdeclstruct {
() => {
// Module: crate
// Provides: {"mkdeclstruct"}
// Dependencies: {}
macro_rules ! mkdeclstruct { (struct $ name : ident $ ($ tt : tt) *) => { println ! ("🔧 WRAPPED: mkdeclstruct! macro called for struct: {}" , stringify ! ($ name)) ; struct $ name $ ($ tt) * } ; }
};
}
