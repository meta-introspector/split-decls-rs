// Generated macro for with_original (macro)
macro_rules! Depcrate_util_with_originalwith_original {
() => {
// Module: crate::util::with_original
// Provides: {"with_original"}
// Dependencies: {}
macro_rules ! with_original { ($ trayt : ident , $ func : ident , $ syn : path) => { impl < T : $ trayt > $ trayt for WithOriginal < T , $ syn > { fn $ func (value : &$ syn) -> Result < Self > { Ok (WithOriginal :: new ($ trayt ::$ func (value) ?, value . clone ())) } } } ; }
};
}
