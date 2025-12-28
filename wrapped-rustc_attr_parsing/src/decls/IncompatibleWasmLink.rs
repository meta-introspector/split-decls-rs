macro_rules! IncompatibleWasmLink {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_incompatible_wasm_link)] pub (crate) struct IncompatibleWasmLink { # [primary_span] pub span : Span , }
    };
}

IncompatibleWasmLink!();