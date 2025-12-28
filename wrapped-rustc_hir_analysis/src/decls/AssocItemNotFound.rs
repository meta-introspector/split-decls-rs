macro_rules! deps {
    () => {
        AssocItemNotFoundSugg!();
        AssocItemNotFoundLabel!();
    };
}

macro_rules! AssocItemNotFound {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_analysis_assoc_item_not_found , code = E0220)] pub (crate) struct AssocItemNotFound < 'a > { # [primary_span] pub span : Span , pub assoc_ident : Ident , pub assoc_kind : & 'static str , pub qself : & 'a str , # [subdiagnostic] pub label : Option < AssocItemNotFoundLabel < 'a > > , # [subdiagnostic] pub sugg : Option < AssocItemNotFoundSugg < 'a > > , # [label (hir_analysis_within_macro)] pub within_macro_span : Option < Span > , }
    };
}

AssocItemNotFound!();