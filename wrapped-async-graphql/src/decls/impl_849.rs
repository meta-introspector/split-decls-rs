macro_rules! deps {
    () => {
        InputType!();
        Result!();
        MetaTypeId!();
        Registry!();
        MetaType!();
        Object!();
        InputValueResult!();
        InputValueError!();
        Scalar!();
    };
}

macro_rules! impl_849 {
    () => {
        deps!();
        impl < K , V , S > InputType for HashMap < K , V , S > where K : ToString + FromStr + Eq + Hash + Send + Sync , K :: Err : Display , V : Serialize + DeserializeOwned + Send + Sync , S : Default + BuildHasher + Send + Sync , { type RawValueType = Self ; fn type_name () -> Cow < 'static , str > { Cow :: Borrowed ("JSONObject") } fn create_type_info (registry : & mut Registry) -> String { registry . create_input_type :: < Self , _ > (MetaTypeId :: Scalar , | _ | MetaType :: Scalar { name : < Self as InputType > :: type_name () . to_string () , description : Some ("A scalar that can represent any JSON Object value." . to_string ()) , is_valid : None , visible : None , inaccessible : false , tags : Default :: default () , specified_by_url : None , directive_invocations : Default :: default () , requires_scopes : Default :: default () , }) } fn parse (value : Option < Value >) -> InputValueResult < Self > { let value = value . unwrap_or_default () ; match value { Value :: Object (map) => map . into_iter () . map (| (name , value) | { Ok ((K :: from_str (& name) . map_err (| err | { InputValueError :: < Self > :: custom (format ! ("object key: {}" , err)) }) ? , from_value (value) . map_err (| err | format ! ("object value: {}" , err)) ? ,)) }) . collect :: < Result < _ , _ > > () . map_err (InputValueError :: propagate) , _ => Err (InputValueError :: expected_type (value)) , } } fn to_value (& self) -> Value { let mut map = IndexMap :: new () ; for (name , value) in self { map . insert (Name :: new (name . to_string ()) , to_value (value) . unwrap_or_default () ,) ; } Value :: Object (map) } fn as_raw_value (& self) -> Option < & Self :: RawValueType > { Some (self) } }
    };
}

impl_849!()