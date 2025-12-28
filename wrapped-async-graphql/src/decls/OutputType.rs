macro_rules! deps {
    () => {
        ServerResult!();
        Field!();
        ContextSelectionSet!();
        Registry!();
    };
}

macro_rules! OutputType {
    () => {
        deps!();
        # [doc = " Represents a GraphQL output type."] # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] pub trait OutputType : Send + Sync { # [doc = " Type the name."] fn type_name () -> Cow < 'static , str > ; # [doc = " Qualified typename."] fn qualified_type_name () -> String { format ! ("{}!" , Self :: type_name ()) } # [doc = " Introspection type name"] # [doc = ""] # [doc = " Is the return value of field `__typename`, the interface and union"] # [doc = " should return the current type, and the others return `Type::type_name`."] fn introspection_type_name (& self) -> Cow < 'static , str > { Self :: type_name () } # [doc = " Create type information in the registry and return qualified typename."] fn create_type_info (registry : & mut registry :: Registry) -> String ; # [doc = " Resolve an output value to `async_graphql::Value`."] # [cfg (feature = "boxed-trait")] async fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> ServerResult < Value > ; # [doc = " Resolve an output value to `async_graphql::Value`."] # [cfg (not (feature = "boxed-trait"))] fn resolve (& self , ctx : & ContextSelectionSet < '_ > , field : & Positioned < Field > ,) -> impl Future < Output = ServerResult < Value > > + Send ; }
    };
}

OutputType!();