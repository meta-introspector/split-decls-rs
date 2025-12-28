macro_rules! deps {
    () => {
        ServerResult!();
        Registry!();
        OutputType!();
        ContextSelectionSet!();
        Field!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : OutputType + ? Sized > OutputType for Arc < T > { fn type_name () -> Cow < 'static , str > { T :: type_name () } fn create_type_info (registry : & mut Registry) -> String { T :: create_type_info (registry) } # [allow (clippy :: trivially_copy_pass_by_ref)] async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { T :: resolve (& * * self , ctx , field) . await } }
    };
}

impl_16!()