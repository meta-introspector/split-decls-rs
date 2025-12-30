// Generated macro for overflow (macro)
macro_rules! Depcrate_deoverflow {
() => {
// Module: crate::de
// Provides: {"overflow"}
// Dependencies: {}
macro_rules ! overflow { ($ a : ident * 10 + $ b : ident , $ c : expr) => { match $ c { c => $ a >= c / 10 && ($ a > c / 10 || $ b > c % 10) , } } ; }
};
}
