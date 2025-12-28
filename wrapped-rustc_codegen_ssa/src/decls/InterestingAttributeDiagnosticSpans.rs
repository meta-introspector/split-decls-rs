macro_rules! InterestingAttributeDiagnosticSpans {
    () => {
        # [doc = " Spans that are collected when processing built-in attributes,"] # [doc = " that are useful for emitting diagnostics later."] # [derive (Default)] struct InterestingAttributeDiagnosticSpans { link_ordinal : Option < Span > , sanitize : Option < Span > , inline : Option < Span > , no_mangle : Option < Span > , }
    };
}

InterestingAttributeDiagnosticSpans!();