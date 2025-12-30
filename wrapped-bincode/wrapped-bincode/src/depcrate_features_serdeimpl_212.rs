// Generated macro for impl_212 (impl)
macro_rules! Depcrate_features_serdeimpl_212 {
() => {
// Module: crate::features::serde
// Provides: {"impl_212"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl serde :: de :: Error for crate :: error :: DecodeError { fn custom < T > (msg : T) -> Self where T : core :: fmt :: Display , { use alloc :: string :: ToString ; Self :: OtherString (msg . to_string ()) } }
};
}
