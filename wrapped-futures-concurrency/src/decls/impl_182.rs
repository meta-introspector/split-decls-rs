macro_rules! deps {
    () => {
        TakeConsumer!();
        Consumer!();
        ConsumerState!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < C , Item , Fut > Consumer < Item , Fut > for TakeConsumer < C > where Fut : Future < Output = Item > , C : Consumer < Item , Fut > , { type Output = C :: Output ; async fn send (self : Pin < & mut Self > , future : Fut) -> ConsumerState { let this = self . project () ; * this . count += 1 ; let state = this . inner . send (future) . await ; if this . count >= this . limit { ConsumerState :: Break } else { state } } async fn progress (self : Pin < & mut Self >) -> ConsumerState { let this = self . project () ; this . inner . progress () . await } async fn flush (self : Pin < & mut Self >) -> Self :: Output { let this = self . project () ; this . inner . flush () . await } }
    };
}

impl_182!();