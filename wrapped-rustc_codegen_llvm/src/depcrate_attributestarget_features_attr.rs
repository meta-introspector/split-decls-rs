// Generated macro for target_features_attr (function)
macro_rules! Depcrate_attributestarget_features_attr {
() => {
// Module: crate::attributes
// Provides: {"target_features_attr"}
// Dependencies: {}
# [doc = " Get the `target-features` LLVM attribute."] pub (crate) fn target_features_attr < 'll > (cx : & CodegenCx < 'll , '_ > , function_features : Vec < String > ,) -> Option < & 'll Attribute > { let global_features = cx . tcx . global_backend_features (()) . iter () . map (String :: as_str) ; let function_features = function_features . iter () . map (String :: as_str) ; let target_features = global_features . chain (function_features) . intersperse (",") . collect :: < String > () ; (! target_features . is_empty ()) . then (| | llvm :: CreateAttrStringValue (cx . llcx , "target-features" , & target_features)) }
};
}
