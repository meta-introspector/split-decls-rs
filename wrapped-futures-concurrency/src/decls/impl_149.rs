macro_rules! deps {
    () => {
        ConsumerState!();
        VecConsumer!();
        Consumer!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < Item , Fut > Consumer < Item , Fut > for VecConsumer < '_ , Fut > where Fut : Future < Output = Item > , { type Output = () ; async fn send (self : Pin < & mut Self > , future : Fut) -> super :: ConsumerState { let mut this = self . project () ; this . group . as_mut () . push (future) ; ConsumerState :: Continue } async fn progress (self : Pin < & mut Self >) -> super :: ConsumerState { let mut this = self . project () ; while let Some (item) = this . group . next () . await { this . output . push (item) ; } ConsumerState :: Empty } async fn flush (self : Pin < & mut Self >) -> Self :: Output { let mut this = self . project () ; while let Some (item) = this . group . next () . await { this . output . push (item) ; } } }
    };
}

impl_149!()