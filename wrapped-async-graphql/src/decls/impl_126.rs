macro_rules! deps {
    () => {
        Mutation!();
        ObjectType!();
        SubscriptionType!();
        Query!();
        Subscription!();
        Schema!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < Query , Mutation , Subscription > Default for Schema < Query , Mutation , Subscription > where Query : Default + ObjectType + 'static , Mutation : Default + ObjectType + 'static , Subscription : Default + SubscriptionType + 'static , { fn default () -> Self { Schema :: new (Query :: default () , Mutation :: default () , Subscription :: default () ,) } }
    };
}

impl_126!();