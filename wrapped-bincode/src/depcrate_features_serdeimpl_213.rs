// Generated macro for impl_213 (impl)
macro_rules! Depcrate_features_serdeimpl_213 {
() => {
// Module: crate::features::serde
// Provides: {"impl_213"}
// Dependencies: {}
# [cfg (not (feature = "alloc"))] impl serde :: de :: Error for crate :: error :: DecodeError { fn custom < T > (_ : T) -> Self where T : core :: fmt :: Display , { DecodeError :: CustomError . into () } }
};
}
