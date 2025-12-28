macro_rules! deps {
    () => {
        OutputType!();
        ContextSelectionSet!();
        Registry!();
        Field!();
        ServerResult!();
    };
}

macro_rules! impl_826 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T > OutputType for Cow < '_ , T > where T : OutputType + ToOwned + ? Sized , < T as ToOwned > :: Owned : Send + Sync , { fn type_name () -> Cow < 'static , str > { T :: type_name () } fn create_type_info (registry : & mut registry :: Registry) -> String { < T as OutputType > :: create_type_info (registry) } async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { self . as_ref () . resolve (ctx , field) . await } }
    };
}

impl_826!();