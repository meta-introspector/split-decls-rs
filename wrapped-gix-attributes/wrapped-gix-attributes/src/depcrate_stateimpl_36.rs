// Generated macro for impl_36 (impl)
macro_rules! Depcrate_stateimpl_36 {
() => {
// Module: crate::state
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'a > From < StateRef < 'a > > for State { fn from (s : StateRef < 'a >) -> Self { match s { StateRef :: Value (v) => State :: Value (v . into ()) , StateRef :: Set => State :: Set , StateRef :: Unset => State :: Unset , StateRef :: Unspecified => State :: Unspecified , } } }
};
}
