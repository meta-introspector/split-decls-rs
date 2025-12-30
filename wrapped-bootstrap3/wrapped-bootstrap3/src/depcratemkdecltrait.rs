// Generated macro for mkdecltrait (macro)
macro_rules! Depcratemkdecltrait {
() => {
// Module: crate
// Provides: {"mkdecltrait"}
// Dependencies: {}
macro_rules ! mkdecltrait { (trait $ name : ident $ ($ tt : tt) *) => { println ! ("🔧 WRAPPED: mkdecltrait! macro called for trait: {}" , stringify ! ($ name)) ; trait $ name $ ($ tt) * } ; }
};
}
