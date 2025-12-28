macro_rules! deps {
    () => {
        Subscription!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        # [Subscription (internal)] impl Subscription { async fn values (& self) -> impl Stream < Item = i32 > { futures_util :: stream :: once (async move { 10 }) } }
    };
}

impl_167!();