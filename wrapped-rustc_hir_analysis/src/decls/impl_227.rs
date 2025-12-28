macro_rules! deps {
    () => {
        NestedSpan!();
        FieldAlreadyDeclaredNestedHelp!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl NestedSpan { fn to_field_already_declared_nested_help (& self) -> errors :: FieldAlreadyDeclaredNestedHelp { errors :: FieldAlreadyDeclaredNestedHelp { span : self . span } } }
    };
}

impl_227!();