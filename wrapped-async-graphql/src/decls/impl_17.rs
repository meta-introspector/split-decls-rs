macro_rules! deps {
    () => {
        Registry!();
        InputType!();
        InputValueResult!();
        InputValueError!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < T : InputType > InputType for Arc < T > { type RawValueType = T :: RawValueType ; fn type_name () -> Cow < 'static , str > { T :: type_name () } fn create_type_info (registry : & mut Registry) -> String { T :: create_type_info (registry) } fn parse (value : Option < ConstValue >) -> InputValueResult < Self > { T :: parse (value) . map (Arc :: new) . map_err (InputValueError :: propagate) } fn to_value (& self) -> ConstValue { T :: to_value (& self) } fn as_raw_value (& self) -> Option < & Self :: RawValueType > { self . as_ref () . as_raw_value () } }
    };
}

impl_17!()