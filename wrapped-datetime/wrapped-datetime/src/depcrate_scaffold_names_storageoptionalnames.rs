// Generated macro for OptionalNames (enum)
macro_rules! Depcrate_scaffold_names_storageOptionalNames {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"OptionalNames"}
// Dependencies: {}
# [doc = " This can be extended in the future to support multiple lengths."] # [doc = " For now, this type wraps a symbols object tagged with a single length. See [#4337](https://github.com/unicode-org/icu4x/issues/4337)"] # [derive (Debug , Copy , Clone)] pub (crate) enum OptionalNames < Variables , Payload > { None , SingleLength { variables : Variables , payload : Payload , } , }
};
}
