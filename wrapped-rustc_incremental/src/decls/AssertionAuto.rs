macro_rules! AssertionAuto {
    () => {
        # [derive (Diagnostic)] # [diag (incremental_assertion_auto)] pub (crate) struct AssertionAuto < 'a > { # [primary_span] pub span : Span , pub name : & 'a str , pub e : & 'a str , }
    };
}

AssertionAuto!()