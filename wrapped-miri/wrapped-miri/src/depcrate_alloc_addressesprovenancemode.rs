// Generated macro for ProvenanceMode (enum)
macro_rules! Depcrate_alloc_addressesProvenanceMode {
() => {
// Module: crate::alloc_addresses
// Provides: {"ProvenanceMode"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum ProvenanceMode { # [doc = " We support `expose_provenance`/`with_exposed_provenance` via \"wildcard\" provenance."] # [doc = " However, we warn on `with_exposed_provenance` to alert the user of the precision loss."] Default , # [doc = " Like `Default`, but without the warning."] Permissive , # [doc = " We error on `with_exposed_provenance`, ensuring no precision loss."] Strict , }
};
}
