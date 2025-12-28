macro_rules! deps {
    () => {
        LabelSource!();
        BindingId!();
        MacroCallPtr!();
        FormatTemplate!();
        ExprSource!();
        LabelId!();
        Expr!();
        FieldSource!();
        LifetimeSource!();
        Pat!();
        ExprOrPatId!();
        PatId!();
        PatFieldSource!();
        Binding!();
        PatSource!();
        HygieneId!();
        TypeSource!();
        ExprOrPatSource!();
        Label!();
        ExprId!();
        ExpressionStoreDiagnostics!();
    };
}

macro_rules! ExpressionStoreBuilder {
    () => {
        deps!();
        # [doc = " The body of an item (function, const etc.)."] # [derive (Debug , Eq , PartialEq , Default)] pub struct ExpressionStoreBuilder { pub exprs : Arena < Expr > , pub pats : Arena < Pat > , pub bindings : Arena < Binding > , pub labels : Arena < Label > , pub lifetimes : Arena < LifetimeRef > , pub binding_owners : FxHashMap < BindingId , ExprId > , pub types : Arena < TypeRef > , block_scopes : Vec < BlockId > , ident_hygiene : FxHashMap < ExprOrPatId , HygieneId > , expr_map : FxHashMap < ExprSource , ExprOrPatId > , expr_map_back : ArenaMap < ExprId , ExprOrPatSource > , pat_map : FxHashMap < PatSource , ExprOrPatId > , pat_map_back : ArenaMap < PatId , ExprOrPatSource > , label_map : FxHashMap < LabelSource , LabelId > , label_map_back : ArenaMap < LabelId , LabelSource > , types_map_back : ArenaMap < TypeRefId , TypeSource > , types_map : FxHashMap < TypeSource , TypeRefId > , lifetime_map_back : ArenaMap < LifetimeRefId , LifetimeSource > , lifetime_map : FxHashMap < LifetimeSource , LifetimeRefId > , binding_definitions : ArenaMap < BindingId , SmallVec < PatId , { 2 * size_of :: < usize > () / size_of :: < PatId > () } > > , # [doc = " We don't create explicit nodes for record fields (`S { record_field: 92 }`)."] # [doc = " Instead, we use id of expression (`92`) to identify the field."] field_map_back : FxHashMap < ExprId , FieldSource > , pat_field_map_back : FxHashMap < PatId , PatFieldSource > , template_map : Option < Box < FormatTemplate > > , expansions : FxHashMap < InFile < MacroCallPtr > , MacroCallId > , # [doc = " Diagnostics accumulated during lowering. These contain `AstPtr`s and so are stored in"] # [doc = " the source map (since they're just as volatile)."] pub (crate) diagnostics : Vec < ExpressionStoreDiagnostics > , }
    };
}

ExpressionStoreBuilder!()