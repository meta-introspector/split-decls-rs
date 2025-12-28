macro_rules! deps {
    () => {
        BoxResolverFn!();
        TypeRef!();
        InputValue!();
        Deprecation!();
    };
}

macro_rules! SubscriptionField {
    () => {
        deps!();
        # [doc = " A GraphQL subscription field"] pub struct SubscriptionField { pub (crate) name : String , pub (crate) description : Option < String > , pub (crate) arguments : IndexMap < String , InputValue > , pub (crate) ty : TypeRef , pub (crate) resolver_fn : BoxResolverFn , pub (crate) deprecation : Deprecation , }
    };
}

SubscriptionField!()