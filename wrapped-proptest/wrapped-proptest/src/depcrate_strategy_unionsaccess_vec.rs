// Generated macro for access_vec (macro)
macro_rules! Depcrate_strategy_unionsaccess_vec {
() => {
// Module: crate::strategy::unions
// Provides: {"access_vec"}
// Dependencies: {}
macro_rules ! access_vec { ([$ ($ muta : tt) *] $ dst : ident = $ this : expr , $ ix : expr , $ body : block) => { { let $ dst = &$ ($ muta) * $ this . options [$ ix] ; $ body } } }
};
}
