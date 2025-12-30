// Generated macro for wrap_either (macro)
macro_rules! Depcrate_iteratorwrap_either {
() => {
// Module: crate::iterator
// Provides: {"wrap_either"}
// Dependencies: {}
macro_rules ! wrap_either { ($ value : expr => $ ($ tail : tt) *) => { match $ value { Left (inner) => inner . map (Left) $ ($ tail) *, Right (inner) => inner . map (Right) $ ($ tail) *, } } ; }
};
}
