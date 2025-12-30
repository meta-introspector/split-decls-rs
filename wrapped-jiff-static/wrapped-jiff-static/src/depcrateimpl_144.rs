// Generated macro for impl_144 (impl)
macro_rules! Depcrateimpl_144 {
() => {
// Module: crate
// Provides: {"impl_144"}
// Dependencies: {}
impl TzifTransitionKind { fn quote (& self) -> proc_macro2 :: TokenStream { match * self { TzifTransitionKind :: Unambiguous => quote ! { jiff :: shared :: TzifTransitionKind :: Unambiguous } , TzifTransitionKind :: Gap => quote ! { jiff :: shared :: TzifTransitionKind :: Gap } , TzifTransitionKind :: Fold => quote ! { jiff :: shared :: TzifTransitionKind :: Fold } , } } }
};
}
