macro_rules! LinkOrdinalOutOfRange {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_link_ordinal_out_of_range)] # [note] pub (crate) struct LinkOrdinalOutOfRange { # [primary_span] pub span : Span , pub ordinal : u128 , }
    };
}

LinkOrdinalOutOfRange!()