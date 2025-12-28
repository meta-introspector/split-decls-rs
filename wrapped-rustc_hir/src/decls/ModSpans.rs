macro_rules! ModSpans {
    () => {
        # [derive (Copy , Clone , Debug , HashStable_Generic)] pub struct ModSpans { # [doc = " A span from the first token past `{` to the last token until `}`."] # [doc = " For `mod foo;`, the inner span ranges from the first token"] # [doc = " to the last token in the external file."] pub inner_span : Span , pub inject_use_span : Span , }
    };
}

ModSpans!()