macro_rules! deps {
    () => {
        MetaBadDelimSugg!();
    };
}

macro_rules! MetaBadDelim {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (attr_parsing_meta_bad_delim)] pub (crate) struct MetaBadDelim { # [primary_span] pub span : Span , # [subdiagnostic] pub sugg : MetaBadDelimSugg , }
    };
}

MetaBadDelim!()