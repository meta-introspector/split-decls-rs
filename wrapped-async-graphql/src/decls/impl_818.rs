macro_rules! deps {
    () => {
        MetaTypeId!();
        Upload!();
        InputValueResult!();
        InputValueError!();
        MetaType!();
        InputType!();
        Registry!();
        Scalar!();
    };
}

macro_rules! impl_818 {
    () => {
        deps!();
        impl InputType for Upload { type RawValueType = Self ; fn type_name () -> Cow < 'static , str > { Cow :: Borrowed ("Upload") } fn create_type_info (registry : & mut registry :: Registry) -> String { registry . create_input_type :: < Self , _ > (MetaTypeId :: Scalar , | _ | registry :: MetaType :: Scalar { name : Self :: type_name () . to_string () , description : None , is_valid : Some (Arc :: new (| value | matches ! (value , Value :: String (_)))) , visible : None , inaccessible : false , tags : Default :: default () , specified_by_url : Some ("https://github.com/jaydenseric/graphql-multipart-request-spec" . to_string () ,) , directive_invocations : Default :: default () , requires_scopes : Default :: default () , }) } fn parse (value : Option < Value >) -> InputValueResult < Self > { const PREFIX : & str = "#__graphql_file__:" ; let value = value . unwrap_or_default () ; if let Value :: String (s) = & value { if let Some (filename) = s . strip_prefix (PREFIX) { return Ok (Upload (filename . parse :: < usize > () . unwrap ())) ; } } Err (InputValueError :: expected_type (value)) } fn to_value (& self) -> Value { Value :: Null } fn as_raw_value (& self) -> Option < & Self :: RawValueType > { Some (self) } }
    };
}

impl_818!();