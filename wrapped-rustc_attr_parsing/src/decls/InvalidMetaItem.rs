macro_rules! deps {
    () => {
        InvalidMetaItemQuoteIdentSugg!();
        InvalidMetaItemRemoveNegSugg!();
    };
}

macro_rules! InvalidMetaItem {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (attr_parsing_invalid_meta_item)] pub (crate) struct InvalidMetaItem { # [primary_span] pub span : Span , pub descr : String , # [subdiagnostic] pub quote_ident_sugg : Option < InvalidMetaItemQuoteIdentSugg > , # [subdiagnostic] pub remove_neg_sugg : Option < InvalidMetaItemRemoveNegSugg > , }
    };
}

InvalidMetaItem!()