macro_rules! deps {
    () => {
        EnumerateFuture!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < FutT , T > Future for EnumerateFuture < FutT , T > where FutT : Future < Output = T > , { type Output = (usize , T) ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if * this . done { panic ! ("future has already been polled to completion once") ; } let item = ready ! (this . fut_t . poll (cx)) ; * this . done = true ; Poll :: Ready ((* this . count , item)) } }
    };
}

impl_133!()