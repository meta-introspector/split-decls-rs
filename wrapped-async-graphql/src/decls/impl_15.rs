macro_rules! deps {
    () => {
        InputType!();
        Registry!();
        InputValueResult!();
        InputValueError!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T : InputType > InputType for Box < T > { type RawValueType = T :: RawValueType ; fn type_name () -> Cow < 'static , str > { T :: type_name () } fn create_type_info (registry : & mut Registry) -> String { T :: create_type_info (registry) } fn parse (value : Option < ConstValue >) -> InputValueResult < Self > { T :: parse (value) . map (Box :: new) . map_err (InputValueError :: propagate) } fn to_value (& self) -> ConstValue { T :: to_value (& self) } fn as_raw_value (& self) -> Option < & Self :: RawValueType > { self . as_ref () . as_raw_value () } }
    };
}

impl_15!();