macro_rules! deps {
    () => {
        Registry!();
        ContextSelectionSet!();
        OutputType!();
        Field!();
        ServerResult!();
    };
}

macro_rules! impl_902 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl OutputType for str { fn type_name () -> Cow < 'static , str > { Cow :: Borrowed ("String") } fn create_type_info (registry : & mut registry :: Registry) -> String { < String as OutputType > :: create_type_info (registry) } async fn resolve (& self , _ : & ContextSelectionSet < '_ > , _field : & Positioned < Field > ,) -> ServerResult < Value > { Ok (Value :: String (self . to_string ())) } }
    };
}

impl_902!()