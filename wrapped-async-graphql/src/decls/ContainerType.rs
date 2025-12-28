macro_rules! deps {
    () => {
        ContextSelectionSet!();
        EmptyMutation!();
        OutputType!();
        Fields!();
        Context!();
        ServerResult!();
    };
}

macro_rules! ContainerType {
    () => {
        deps!();
        # [doc = " Represents a GraphQL container object."] # [doc = ""] # [doc = " This helper trait allows the type to call `resolve_container` on itself in"] # [doc = " its `OutputType::resolve` implementation."] # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] pub trait ContainerType : OutputType { # [doc = " This function returns true of type `EmptyMutation` only."] # [doc (hidden)] fn is_empty () -> bool { false } # [doc = " Resolves a field value and outputs it as a json value"] # [doc = " `async_graphql::Value`."] # [doc = ""] # [doc = " If the field was not found returns None."] # [cfg (feature = "boxed-trait")] async fn resolve_field (& self , ctx : & Context < '_ >) -> ServerResult < Option < Value > > ; # [doc = " Resolves a field value and outputs it as a json value"] # [doc = " `async_graphql::Value`."] # [doc = ""] # [doc = " If the field was not found returns None."] # [cfg (not (feature = "boxed-trait"))] fn resolve_field (& self , ctx : & Context < '_ > ,) -> impl Future < Output = ServerResult < Option < Value > > > + Send ; # [doc = " Collect all the fields of the container that are queried in the"] # [doc = " selection set."] # [doc = ""] # [doc = " Objects do not have to override this, but interfaces and unions must"] # [doc = " call it on their internal type."] fn collect_all_fields < 'a > (& 'a self , ctx : & ContextSelectionSet < 'a > , fields : & mut Fields < 'a > ,) -> ServerResult < () > where Self : Send + Sync , { fields . add_set (ctx , self) } # [doc = " Find the GraphQL entity with the given name from the parameter."] # [doc = ""] # [doc = " Objects should override this in case they are the query root."] # [cfg (feature = "boxed-trait")] async fn find_entity (& self , _ : & Context < '_ > , _ : & Value) -> ServerResult < Option < Value > > { Ok (None) } # [doc = " Find the GraphQL entity with the given name from the parameter."] # [doc = ""] # [doc = " Objects should override this in case they are the query root."] # [cfg (not (feature = "boxed-trait"))] fn find_entity (& self , _ : & Context < '_ > , _params : & Value ,) -> impl Future < Output = ServerResult < Option < Value > > > + Send { async { Ok (None) } } }
    };
}

ContainerType!();