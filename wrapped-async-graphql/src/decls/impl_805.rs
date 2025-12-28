macro_rules! deps {
    () => {
        ContextSelectionSet!();
        ServerResult!();
        IntrospectionMode!();
        QueryRoot!();
        OutputType!();
        ObjectType!();
        Registry!();
        Field!();
    };
}

macro_rules! impl_805 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : ObjectType > OutputType for QueryRoot < T > { fn type_name () -> Cow < 'static , str > { T :: type_name () } fn create_type_info (registry : & mut registry :: Registry) -> String { let root = T :: create_type_info (registry) ; if matches ! (registry . introspection_mode , IntrospectionMode :: Enabled | IntrospectionMode :: IntrospectionOnly) { registry . create_introspection_types () ; } root } async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , _field : & Positioned < Field > ,) -> ServerResult < Value > { resolve_container (ctx , self) . await } }
    };
}

impl_805!()