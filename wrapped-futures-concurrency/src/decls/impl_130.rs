macro_rules! deps {
    () => {
        EnumerateFuture!();
        ConsumerState!();
        Consumer!();
        EnumerateConsumer!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < C , Item , Fut > Consumer < Item , Fut > for EnumerateConsumer < C > where Fut : Future < Output = Item > , C : Consumer < (usize , Item) , EnumerateFuture < Fut , Item > > , { type Output = C :: Output ; async fn send (self : Pin < & mut Self > , future : Fut) -> super :: ConsumerState { let this = self . project () ; let count = * this . count ; * this . count += 1 ; this . inner . send (EnumerateFuture :: new (future , count)) . await } async fn progress (self : Pin < & mut Self >) -> super :: ConsumerState { let this = self . project () ; this . inner . progress () . await } async fn flush (self : Pin < & mut Self >) -> Self :: Output { let this = self . project () ; this . inner . flush () . await } }
    };
}

impl_130!();