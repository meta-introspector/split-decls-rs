macro_rules! deps {
    () => {
        TypeRef!();
        BoxResolverFn!();
        Deprecation!();
        InputValue!();
    };
}

macro_rules! SubscriptionField {
    () => {
        deps!();
        # [doc = " A GraphQL subscription field"] pub struct SubscriptionField { pub (crate) name : String , pub (crate) description : Option < String > , pub (crate) arguments : IndexMap < String , InputValue > , pub (crate) ty : TypeRef , pub (crate) resolver_fn : BoxResolverFn , pub (crate) deprecation : Deprecation , }
    };
}

SubscriptionField!();