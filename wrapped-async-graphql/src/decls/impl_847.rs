macro_rules! deps {
    () => {
        Registry!();
        ContextSelectionSet!();
        Object!();
        Field!();
        ServerResult!();
        OutputType!();
    };
}

macro_rules! impl_847 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < K , V > OutputType for HashMap < K , V > where K : ToString + Eq + Hash + Send + Sync , V : Serialize + Send + Sync , { fn type_name () -> Cow < 'static , str > { < StdHashMap < K , V > as OutputType > :: type_name () } fn create_type_info (registry : & mut Registry) -> String { < StdHashMap < K , V > as OutputType > :: create_type_info (registry) } async fn resolve (& self , _ctx : & ContextSelectionSet < '_ > , _field : & Positioned < Field > ,) -> ServerResult < Value > { let mut map = IndexMap :: new () ; for (name , value) in self { map . insert (Name :: new (name . to_string ()) , to_value (value) . unwrap_or_default () ,) ; } Ok (Value :: Object (map)) } }
    };
}

impl_847!()