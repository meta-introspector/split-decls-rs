// Generated macro for BodyOrSig (enum)
macro_rules! Depcrate_source_analyzerBodyOrSig {
() => {
// Module: crate::source_analyzer
// Provides: {"BodyOrSig"}
// Dependencies: {}
# [derive (Debug)] pub (crate) enum BodyOrSig < 'db > { Body { def : DefWithBodyId , body : Arc < Body > , source_map : Arc < BodySourceMap > , infer : Option < Arc < InferenceResult < 'db > > > , } , VariantFields { def : VariantId , store : Arc < ExpressionStore > , source_map : Arc < ExpressionStoreSourceMap > , } , Sig { def : GenericDefId , store : Arc < ExpressionStore > , source_map : Arc < ExpressionStoreSourceMap > , } , }
};
}
