macro_rules! deps {
    () => {
        Consumer!();
        MapFuture!();
        MapConsumer!();
        ConsumerState!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < C , F , FutT , T , FutB , B > Consumer < T , FutT > for MapConsumer < C , F , FutT , T , FutB , B > where FutT : Future < Output = T > , C : Consumer < B , MapFuture < F , FutT , T , FutB , B > > , F : Fn (T) -> FutB , F : Clone , FutB : Future < Output = B > , { type Output = C :: Output ; async fn progress (self : Pin < & mut Self >) -> super :: ConsumerState { let this = self . project () ; this . inner . progress () . await } async fn send (self : Pin < & mut Self > , future : FutT) -> super :: ConsumerState { let this = self . project () ; let fut = MapFuture :: new (this . f . clone () , future) ; this . inner . send (fut) . await } async fn flush (self : Pin < & mut Self >) -> Self :: Output { let this = self . project () ; this . inner . flush () . await } }
    };
}

impl_173!();