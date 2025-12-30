// Generated macro for tune_cpu_attr (function)
macro_rules! Depcrate_attributestune_cpu_attr {
() => {
// Module: crate::attributes
// Provides: {"tune_cpu_attr"}
// Dependencies: {}
pub (crate) fn tune_cpu_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> Option < & 'll Attribute > { llvm_util :: tune_cpu (cx . tcx . sess) . map (| tune_cpu | llvm :: CreateAttrStringValue (cx . llcx , "tune-cpu" , tune_cpu)) }
};
}
