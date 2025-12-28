macro_rules! deps {
    () => {
        ServerResult!();
        ResolveFut!();
        Context!();
    };
}

macro_rules! CustomDirective {
    () => {
        deps!();
        # [doc = " Represents a custom directive."] # [async_trait :: async_trait] # [allow (unused_variables)] pub trait CustomDirective : Sync + Send + 'static { # [doc = " Called at resolve field."] async fn resolve_field (& self , ctx : & Context < '_ > , resolve : ResolveFut < '_ > ,) -> ServerResult < Option < Value > > { resolve . await } }
    };
}

CustomDirective!()