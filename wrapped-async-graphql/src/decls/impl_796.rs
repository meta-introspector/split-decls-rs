macro_rules! deps {
    () => {
        ContainerType!();
        ServerResult!();
        MergedObject!();
        Context!();
    };
}

macro_rules! impl_796 {
    () => {
        deps!();
        # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < A , B > ContainerType for MergedObject < A , B > where A : ContainerType , B : ContainerType , { async fn resolve_field (& self , ctx : & Context < '_ >) -> ServerResult < Option < Value > > { match self . 0 . resolve_field (ctx) . await { Ok (Some (value)) => Ok (Some (value)) , Ok (None) => self . 1 . resolve_field (ctx) . await , Err (err) => Err (err) , } } async fn find_entity (& self , ctx : & Context < '_ > , params : & Value) -> ServerResult < Option < Value > > { match self . 0 . find_entity (ctx , params) . await { Ok (Some (value)) => Ok (Some (value)) , Ok (None) => self . 1 . find_entity (ctx , params) . await , Err (err) => Err (err) , } } }
    };
}

impl_796!()