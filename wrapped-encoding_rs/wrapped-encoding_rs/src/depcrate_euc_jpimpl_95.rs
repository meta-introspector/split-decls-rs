// Generated macro for impl_95 (impl)
macro_rules! Depcrate_euc_jpimpl_95 {
() => {
// Module: crate::euc_jp
// Provides: {"impl_95"}
// Dependencies: {}
impl EucJpPending { fn is_none (& self) -> bool { match * self { EucJpPending :: None => true , _ => false , } } fn count (& self) -> usize { match * self { EucJpPending :: None => 0 , EucJpPending :: Jis0208Lead (_) | EucJpPending :: Jis0212Shift | EucJpPending :: HalfWidthKatakana => 1 , EucJpPending :: Jis0212Lead (_) => 2 , } } }
};
}
