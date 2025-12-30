// Generated macro for impl_217 (impl)
macro_rules! Depcrate_features_serdeimpl_217 {
() => {
// Module: crate::features::serde
// Provides: {"impl_217"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl serde :: ser :: Error for crate :: error :: EncodeError { fn custom < T > (msg : T) -> Self where T : core :: fmt :: Display , { use alloc :: string :: ToString ; Self :: OtherString (msg . to_string ()) } }
};
}
