macro_rules! deps {
    () => {
        ContextSelectionSet!();
        OutputType!();
        Field!();
        ServerResult!();
        Registry!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : OutputType + ? Sized > OutputType for Box < T > { fn type_name () -> Cow < 'static , str > { T :: type_name () } fn create_type_info (registry : & mut Registry) -> String { T :: create_type_info (registry) } # [cfg (feature = "boxed-trait")] async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > { T :: resolve (self . as_ref () , ctx , field) . await } # [allow (clippy :: trivially_copy_pass_by_ref)] # [cfg (not (feature = "boxed-trait"))] fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> impl Future < Output = ServerResult < Value > > + Send { T :: resolve (self . as_ref () , ctx , field) } }
    };
}

impl_14!()