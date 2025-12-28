macro_rules! deps {
    () => {
        Schema!();
    };
}

macro_rules! EmptySubscription {
    () => {
        deps!();
        # [doc = " Empty subscription"] # [doc = ""] # [doc = " Only the parameters used to construct the Schema, representing an"] # [doc = " unconfigured subscription."] # [derive (Default , Copy , Clone)] pub struct EmptySubscription ;
    };
}

EmptySubscription!();