macro_rules! ConsiderDereferencing {
    () => {
        # [derive (Subdiagnostic)] # [multipart_suggestion (const_eval_consider_dereferencing , applicability = "machine-applicable")] pub struct ConsiderDereferencing { pub deref : String , # [suggestion_part (code = "{deref}")] pub span : Span , # [suggestion_part (code = "{deref}")] pub rhs_span : Span , }
    };
}

ConsiderDereferencing!()