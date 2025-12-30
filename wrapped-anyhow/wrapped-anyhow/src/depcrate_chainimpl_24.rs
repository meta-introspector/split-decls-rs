// Generated macro for impl_24 (impl)
macro_rules! Depcrate_chainimpl_24 {
() => {
// Module: crate::chain
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a > Chain < 'a > { # [cold] pub fn new (head : & 'a (dyn StdError + 'static)) -> Self { Chain { state : ChainState :: Linked { next : Some (head) } , } } }
};
}
