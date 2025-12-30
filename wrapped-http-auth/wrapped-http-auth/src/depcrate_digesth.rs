// Generated macro for h (function)
macro_rules! Depcrate_digesth {
() => {
// Module: crate::digest
// Provides: {"h"}
// Dependencies: {}
fn h < D : Digest > (mut d : D , items : & [& [u8]]) -> String { for i in items { d . update (i) ; } hex :: encode (d . finalize ()) }
};
}
