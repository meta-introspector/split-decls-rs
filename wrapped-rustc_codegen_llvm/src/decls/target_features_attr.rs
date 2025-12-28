macro_rules! deps {
    () => {
        CreateAttrStringValue!();
        CodegenCx!();
    };
}

macro_rules! target_features_attr {
    () => {
        deps!();
        # [doc = " Get the `target-features` LLVM attribute."] pub (crate) fn target_features_attr < 'll > (cx : & CodegenCx < 'll , '_ > , function_features : Vec < String > ,) -> Option < & 'll Attribute > { let global_features = cx . tcx . global_backend_features (()) . iter () . map (String :: as_str) ; let function_features = function_features . iter () . map (String :: as_str) ; let target_features = global_features . chain (function_features) . intersperse (",") . collect :: < String > () ; (! target_features . is_empty ()) . then (| | llvm :: CreateAttrStringValue (cx . llcx , "target-features" , & target_features)) }
    };
}

target_features_attr!()