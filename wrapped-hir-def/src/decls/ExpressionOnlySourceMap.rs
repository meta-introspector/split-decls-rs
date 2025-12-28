macro_rules! deps {
    () => {
        PatFieldSource!();
        LabelId!();
        FieldSource!();
        FormatTemplate!();
        PatSource!();
        ExprSource!();
        ExprOrPatId!();
        BindingId!();
        ExprId!();
        ExprOrPatSource!();
        MacroCallPtr!();
        PatId!();
        LabelSource!();
        ExpressionStoreDiagnostics!();
    };
}

macro_rules! ExpressionOnlySourceMap {
    () => {
        deps!();
        # [derive (Debug , Eq , Default)] struct ExpressionOnlySourceMap { expr_map : FxHashMap < ExprSource , ExprOrPatId > , expr_map_back : ArenaMap < ExprId , ExprOrPatSource > , pat_map : FxHashMap < PatSource , ExprOrPatId > , pat_map_back : ArenaMap < PatId , ExprOrPatSource > , label_map : FxHashMap < LabelSource , LabelId > , label_map_back : ArenaMap < LabelId , LabelSource > , binding_definitions : ArenaMap < BindingId , SmallVec < PatId , { 2 * size_of :: < usize > () / size_of :: < PatId > () } > > , # [doc = " We don't create explicit nodes for record fields (`S { record_field: 92 }`)."] # [doc = " Instead, we use id of expression (`92`) to identify the field."] field_map_back : FxHashMap < ExprId , FieldSource > , pat_field_map_back : FxHashMap < PatId , PatFieldSource > , template_map : Option < Box < FormatTemplate > > , expansions : FxHashMap < InFile < MacroCallPtr > , MacroCallId > , # [doc = " Diagnostics accumulated during lowering. These contain `AstPtr`s and so are stored in"] # [doc = " the source map (since they're just as volatile)."] diagnostics : ThinVec < ExpressionStoreDiagnostics > , }
    };
}

ExpressionOnlySourceMap!();