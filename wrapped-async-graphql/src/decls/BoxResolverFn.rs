macro_rules! deps {
    () => {
        ResolverContext!();
        SubscriptionFieldFuture!();
    };
}

macro_rules! BoxResolverFn {
    () => {
        deps!();
        type BoxResolverFn = Arc < (dyn for < 'a > Fn (ResolverContext < 'a >) -> SubscriptionFieldFuture < 'a > + Send + Sync) > ;
    };
}

BoxResolverFn!();