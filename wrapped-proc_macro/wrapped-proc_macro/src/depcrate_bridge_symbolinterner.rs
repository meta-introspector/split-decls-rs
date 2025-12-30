// Generated macro for Interner (struct)
macro_rules! Depcrate_bridge_symbolInterner {
() => {
// Module: crate::bridge::symbol
// Provides: {"Interner"}
// Dependencies: {}
# [doc = " Basic interner for a `Symbol`, inspired by the one in `rustc_span`."] struct Interner { arena : arena :: Arena , names : fxhash :: FxHashMap < & 'static str , Symbol > , strings : Vec < & 'static str > , sym_base : NonZero < u32 > , }
};
}
