// Generated macro for target_cpu_attr (function)
macro_rules! Depcrate_attributestarget_cpu_attr {
() => {
// Module: crate::attributes
// Provides: {"target_cpu_attr"}
// Dependencies: {}
pub (crate) fn target_cpu_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> & 'll Attribute { let target_cpu = llvm_util :: target_cpu (cx . tcx . sess) ; llvm :: CreateAttrStringValue (cx . llcx , "target-cpu" , target_cpu) }
};
}
