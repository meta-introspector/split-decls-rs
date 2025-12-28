macro_rules! CfgPredicateIdentifier {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_cfg_predicate_identifier)] pub (crate) struct CfgPredicateIdentifier { # [primary_span] pub span : Span , }
    };
}

CfgPredicateIdentifier!()