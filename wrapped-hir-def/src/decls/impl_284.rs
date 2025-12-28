macro_rules! deps {
    () => {
        ExpressionOnlySourceMap!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        impl PartialEq for ExpressionOnlySourceMap { fn eq (& self , other : & Self) -> bool { let Self { expr_map : _ , expr_map_back , pat_map : _ , pat_map_back , label_map : _ , label_map_back , binding_definitions : _ , field_map_back : _ , pat_field_map_back : _ , template_map , expansions , diagnostics , } = self ; * expr_map_back == other . expr_map_back && * pat_map_back == other . pat_map_back && * label_map_back == other . label_map_back && * template_map == other . template_map && * expansions == other . expansions && * diagnostics == other . diagnostics } }
    };
}

impl_284!();