macro_rules! deps {
    () => {
        BoxResolveFut!();
    };
}

macro_rules! SubscriptionFieldFuture {
    () => {
        deps!();
        # [doc = " A future that returned from field resolver"] pub struct SubscriptionFieldFuture < 'a > (pub (crate) BoxResolveFut < 'a >) ;
    };
}

SubscriptionFieldFuture!()