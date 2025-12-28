macro_rules! deps {
    () => {
        ContainerType!();
        ServerResult!();
        Context!();
    };
}

macro_rules! impl_677 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < T : ContainerType + ? Sized > ContainerType for & T { async fn resolve_field (& self , ctx : & Context < '_ >) -> ServerResult < Option < Value > > { T :: resolve_field (* self , ctx) . await } async fn find_entity (& self , ctx : & Context < '_ > , params : & Value) -> ServerResult < Option < Value > > { T :: find_entity (* self , ctx , params) . await } }
    };
}

impl_677!()