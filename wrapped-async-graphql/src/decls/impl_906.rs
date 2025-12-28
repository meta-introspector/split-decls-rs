macro_rules! deps {
    () => {
        Registry!();
        ContextSelectionSet!();
        OutputType!();
        ServerResult!();
        Field!();
    };
}

macro_rules! impl_906 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : OutputType > OutputType for RwLock < T > { fn type_name () -> Cow < 'static , str > { T :: type_name () } fn create_type_info (registry : & mut registry :: Registry) -> String { < T as OutputType > :: create_type_info (registry) } async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { self . read () . await . resolve (ctx , field) . await } }
    };
}

impl_906!()