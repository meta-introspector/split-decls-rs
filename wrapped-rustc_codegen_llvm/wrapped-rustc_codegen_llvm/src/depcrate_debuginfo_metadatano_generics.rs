// Generated macro for NO_GENERICS (const)
macro_rules! Depcrate_debuginfo_metadataNO_GENERICS {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"NO_GENERICS"}
// Dependencies: {}
# [doc = " A function that returns an empty list of generic parameter debuginfo nodes."] const NO_GENERICS : for < 'll > fn (& CodegenCx < 'll , '_ >) -> SmallVec < Option < & 'll DIType > > = | _ | SmallVec :: new () ;
};
}
