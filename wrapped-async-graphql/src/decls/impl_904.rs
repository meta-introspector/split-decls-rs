macro_rules! deps {
    () => {
        OutputType!();
        Registry!();
        Field!();
        ContextSelectionSet!();
        ServerResult!();
    };
}

macro_rules! impl_904 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : OutputType > OutputType for Mutex < T > { fn type_name () -> Cow < 'static , str > { T :: type_name () } fn create_type_info (registry : & mut registry :: Registry) -> String { < T as OutputType > :: create_type_info (registry) } async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { self . lock () . await . resolve (ctx , field) . await } }
    };
}

impl_904!()