macro_rules! deps {
    () => {
        EmptyMutation!();
        ContainerType!();
        Context!();
        ServerResult!();
    };
}

macro_rules! impl_753 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl ContainerType for EmptyMutation { fn is_empty () -> bool { true } async fn resolve_field (& self , _ctx : & Context < '_ >) -> ServerResult < Option < Value > > { Ok (None) } }
    };
}

impl_753!();