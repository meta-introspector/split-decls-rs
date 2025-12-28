macro_rules! deps {
    () => {
        LimitConsumer!();
        Consumer!();
        ConsumerState!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < C , Item , Fut > Consumer < Item , Fut > for LimitConsumer < C > where Fut : Future < Output = Item > , C : Consumer < Item , Fut > , { type Output = C :: Output ; async fn send (self : Pin < & mut Self > , future : Fut) -> super :: ConsumerState { let this = self . project () ; this . inner . send (future) . await } async fn progress (self : Pin < & mut Self >) -> super :: ConsumerState { let this = self . project () ; this . inner . progress () . await } async fn flush (self : Pin < & mut Self >) -> Self :: Output { let this = self . project () ; this . inner . flush () . await } }
    };
}

impl_167!()