// Generated macro for ParDrainProducer (struct)
macro_rules! Depcrate_external_trait_impls_rayon_rawParDrainProducer {
() => {
// Module: crate::external_trait_impls::rayon::raw
// Provides: {"ParDrainProducer"}
// Dependencies: {}
# [doc = " Producer which will consume all elements in the range, even if it is dropped"] # [doc = " halfway through."] struct ParDrainProducer < T > { iter : RawIterRange < T > , }
};
}
