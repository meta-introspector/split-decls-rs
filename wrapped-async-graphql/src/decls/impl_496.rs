macro_rules! deps {
    () => {
        TypeRef!();
        ResolverContext!();
        Deprecation!();
        SubscriptionFieldFuture!();
        InputValue!();
        SubscriptionField!();
    };
}

macro_rules! impl_496 {
    () => {
        deps!();
        impl SubscriptionField { # [doc = " Create a GraphQL subscription field"] pub fn new < N , T , F > (name : N , ty : T , resolver_fn : F) -> Self where N : Into < String > , T : Into < TypeRef > , F : for < 'a > Fn (ResolverContext < 'a >) -> SubscriptionFieldFuture < 'a > + Send + Sync + 'static , { Self { name : name . into () , description : None , arguments : Default :: default () , ty : ty . into () , resolver_fn : Arc :: new (resolver_fn) , deprecation : Deprecation :: NoDeprecated , } } impl_set_description ! () ; impl_set_deprecation ! () ; # [doc = " Add an argument to the subscription field"] # [inline] pub fn argument (mut self , input_value : InputValue) -> Self { self . arguments . insert (input_value . name . clone () , input_value) ; self } }
    };
}

impl_496!()