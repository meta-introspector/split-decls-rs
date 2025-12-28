macro_rules! deps {
    () => {
        InputValueError!();
        Registry!();
        InputType!();
        OutputType!();
        InputValueResult!();
    };
}

macro_rules! impl_input_string_for_smart_ptr {
    () => {
        deps!();
        macro_rules ! impl_input_string_for_smart_ptr { ($ ty : ty) => { impl InputType for $ ty { type RawValueType = Self ; fn type_name () -> Cow <'static , str > { Cow :: Borrowed ("String") } fn create_type_info (registry : & mut Registry) -> String { < String as OutputType >:: create_type_info (registry) } fn parse (value : Option < Value >) -> InputValueResult < Self > { let value = value . unwrap_or_default () ; match value { Value :: String (s) => Ok (s . into ()) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { Value :: String (self . to_string ()) } fn as_raw_value (& self) -> Option <& Self :: RawValueType > { Some (self) } } } ; }
    };
}

impl_input_string_for_smart_ptr!()