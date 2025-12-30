// Generated macro for impl_35 (impl)
macro_rules! Depcrate_stateimpl_35 {
() => {
// Module: crate::state
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'a > State { # [doc = " Turn ourselves into our ref-type."] pub fn as_ref (& 'a self) -> StateRef < 'a > { match self { State :: Value (v) => StateRef :: Value (v . as_ref ()) , State :: Set => StateRef :: Set , State :: Unset => StateRef :: Unset , State :: Unspecified => StateRef :: Unspecified , } } }
};
}
