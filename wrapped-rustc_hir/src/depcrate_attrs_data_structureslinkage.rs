// Generated macro for Linkage (enum)
macro_rules! Depcrate_attrs_data_structuresLinkage {
() => {
// Module: crate::attrs::data_structures
// Provides: {"Linkage"}
// Dependencies: {}
# [doc = " Possible values for the `#[linkage]` attribute, allowing to specify the"] # [doc = " linkage type for a `MonoItem`."] # [doc = ""] # [doc = " See <https://llvm.org/docs/LangRef.html#linkage-types> for more details about these variants."] # [derive (Encodable , Decodable , Clone , Copy , Debug , PartialEq , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub enum Linkage { AvailableExternally , Common , ExternalWeak , External , Internal , LinkOnceAny , LinkOnceODR , WeakAny , WeakODR , }
};
}
