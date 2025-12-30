// Generated macro for impl_218 (impl)
macro_rules! Depcrate_features_serdeimpl_218 {
() => {
// Module: crate::features::serde
// Provides: {"impl_218"}
// Dependencies: {}
# [cfg (not (feature = "alloc"))] impl serde :: ser :: Error for crate :: error :: EncodeError { fn custom < T > (_ : T) -> Self where T : core :: fmt :: Display , { EncodeError :: CustomError . into () } }
};
}
