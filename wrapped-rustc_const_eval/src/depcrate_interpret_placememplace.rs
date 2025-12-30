// Generated macro for MemPlace (struct)
macro_rules! Depcrate_interpret_placeMemPlace {
() => {
// Module: crate::interpret::place
// Provides: {"MemPlace"}
// Dependencies: {}
# [derive (Copy , Clone , Hash , PartialEq , Eq , Debug)] pub (super) struct MemPlace < Prov : Provenance = CtfeProvenance > { # [doc = " The pointer can be a pure integer, with the `None` provenance."] pub ptr : Pointer < Option < Prov > > , # [doc = " Metadata for unsized places. Interpretation is up to the type."] # [doc = " Must not be present for sized types, but can be missing for unsized types"] # [doc = " (e.g., `extern type`)."] pub meta : MemPlaceMeta < Prov > , # [doc = " Stores whether this place was created based on a sufficiently aligned pointer."] misaligned : Option < Misalignment > , }
};
}
