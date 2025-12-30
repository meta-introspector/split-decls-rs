// Generated macro for nojumptables_attr (function)
macro_rules! Depcrate_attributesnojumptables_attr {
() => {
// Module: crate::attributes
// Provides: {"nojumptables_attr"}
// Dependencies: {}
fn nojumptables_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> Option < & 'll Attribute > { if ! cx . sess () . opts . unstable_opts . no_jump_tables { return None ; } Some (llvm :: CreateAttrStringValue (cx . llcx , "no-jump-tables" , "true")) }
};
}
