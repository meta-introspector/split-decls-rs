macro_rules! TypeName {
    () => {
        # [doc = " Used to specify the GraphQL Type name."] pub trait TypeName : Send + Sync { # [doc = " Returns a GraphQL type name."] fn type_name () -> Cow < 'static , str > ; }
    };
}

TypeName!()