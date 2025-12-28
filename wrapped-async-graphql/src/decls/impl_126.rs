macro_rules! deps {
    () => {
        ObjectType!();
        Subscription!();
        Query!();
        Mutation!();
        Schema!();
        SubscriptionType!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < Query , Mutation , Subscription > Default for Schema < Query , Mutation , Subscription > where Query : Default + ObjectType + 'static , Mutation : Default + ObjectType + 'static , Subscription : Default + SubscriptionType + 'static , { fn default () -> Self { Schema :: new (Query :: default () , Mutation :: default () , Subscription :: default () ,) } }
    };
}

impl_126!()