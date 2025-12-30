// Generated macro for NonstandardWitSection (struct)
macro_rules! Depcrate_wit_standardNonstandardWitSection {
() => {
// Module: crate::wit::standard
// Provides: {"NonstandardWitSection"}
// Dependencies: {}
# [derive (Default , Debug)] pub struct NonstandardWitSection { # [doc = " A list of adapter functions, keyed by their id."] # [doc = ""] # [doc = " This map is iterated over in multiple places, so we use an ordered map"] # [doc = " to ensure that the order of iteration is deterministic. This map affects"] # [doc = " all parts of the generated code, so it's important to get this right."] pub adapters : BTreeMap < AdapterId , Adapter > , # [doc = " A list of pairs for adapter functions that implement core Wasm imports."] pub implements : Vec < (ImportId , FunctionId , AdapterId) > , # [doc = " A list of adapter functions and the names they're exported under."] pub exports : Vec < (ExportId , AdapterId) > , }
};
}
