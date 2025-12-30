// Generated macro for mkdeclfn (macro)
macro_rules! Depcratemkdeclfn {
() => {
// Module: crate
// Provides: {"mkdeclfn"}
// Dependencies: {}
macro_rules ! mkdeclfn { (fn $ name : ident $ ($ tt : tt) *) => { println ! ("🔧 WRAPPED: mkdeclfn! called for function: {}" , stringify ! ($ name)) ; println ! ("🔍 TRACE: → {} entry" , stringify ! ($ name)) ; fn $ name $ ($ tt) * { println ! ("🔍 TRACE: ← {} exit" , stringify ! ($ name)) ; } } ; }
};
}
