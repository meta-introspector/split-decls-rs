macro_rules! deps {
    () => {
        InputType!();
        MetaType!();
        MetaTypeId!();
        Scalar!();
        InputValueResult!();
        Registry!();
    };
}

macro_rules! impl_780 {
    () => {
        deps!();
        impl InputType for serde_json :: Value { type RawValueType = serde_json :: Value ; fn type_name () -> Cow < 'static , str > { Cow :: Borrowed ("JSON") } fn create_type_info (registry : & mut Registry) -> String { registry . create_input_type :: < serde_json :: Value , _ > (MetaTypeId :: Scalar , | _ | { MetaType :: Scalar { name : < Self as InputType > :: type_name () . to_string () , description : Some ("A scalar that can represent any JSON value." . to_string ()) , is_valid : None , visible : None , inaccessible : false , tags : Default :: default () , specified_by_url : None , directive_invocations : Default :: default () , requires_scopes : Default :: default () , } }) } fn parse (value : Option < Value >) -> InputValueResult < Self > { Ok (from_value (value . unwrap_or_default ()) ?) } fn to_value (& self) -> Value { Value :: String (self . to_string ()) } fn as_raw_value (& self) -> Option < & Self :: RawValueType > { Some (& self) } }
    };
}

impl_780!();