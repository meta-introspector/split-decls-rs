macro_rules! deps {
    () => {
        FieldValue!();
        Result!();
        SubscriptionFieldFuture!();
    };
}

macro_rules! impl_493 {
    () => {
        deps!();
        impl < 'a > SubscriptionFieldFuture < 'a > { # [doc = " Create a ResolverFuture"] pub fn new < Fut , S , T > (future : Fut) -> Self where Fut : Future < Output = Result < S > > + Send + 'a , S : Stream < Item = Result < T > > + Send + 'a , T : Into < FieldValue < 'a > > + Send + 'a , { Self (async move { let res = future . await ? . map_ok (Into :: into) ; Ok (res . boxed ()) } . boxed () ,) } }
    };
}

impl_493!()