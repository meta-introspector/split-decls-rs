macro_rules! InvalidAttrStyle {
    () => {
        # [derive (LintDiagnostic)] # [diag (attr_parsing_invalid_style)] pub (crate) struct InvalidAttrStyle { pub name : AttrPath , pub is_used_as_inner : bool , # [note] pub target_span : Option < Span > , pub target : Target , }
    };
}

InvalidAttrStyle!()