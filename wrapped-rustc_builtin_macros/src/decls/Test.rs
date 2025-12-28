macro_rules! Test {
    () => {
        # [derive (Clone)] struct Test { span : Span , ident : Ident , name : Symbol , }
    };
}

Test!();