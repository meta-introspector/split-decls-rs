// Generated macro for Chain (enum)
macro_rules! Depcrate_chainChain {
() => {
// Module: crate::chain
// Provides: {"Chain"}
// Dependencies: {}
pub enum Chain < A , B , C > where A : Future , B : Send + 'static { First (Collapsed < A > , C) , Second (B) , Done , }
};
}
