macro_rules! Condition {
    () => {
        # [derive (Clone , Debug)] pub struct Condition { pub name : Symbol , pub name_span : Span , pub value : Option < Symbol > , pub value_span : Option < Span > , pub span : Span , }
    };
}

Condition!()