// Generated macro for libraries (function)
macro_rules! Depcrate_librarieslibraries {
() => {
// Module: crate::libraries
// Provides: {"libraries"}
// Dependencies: {}
# [doc (hidden)] pub fn libraries () -> BTreeMap < String , BTreeMap < String , CallingConvention > > { let mut libraries = BTreeMap :: new () ; let reader = Reader :: new (expand_input (& ["default"])) ; combine_libraries (& reader , & mut libraries) ; libraries }
};
}
