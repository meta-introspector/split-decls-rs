macro_rules! deps {
    () => {
        SubscriptionField!();
    };
}

macro_rules! Subscription {
    () => {
        deps!();
        # [doc = " A GraphQL subscription type"] # [derive (Debug)] pub struct Subscription { pub (crate) name : String , pub (crate) description : Option < String > , pub (crate) fields : IndexMap < String , SubscriptionField > , }
    };
}

Subscription!();