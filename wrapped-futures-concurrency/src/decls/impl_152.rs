macro_rules! deps {
    () => {
        ResultVecConsumer!();
        ConsumerState!();
        Consumer!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < Fut , T , E > Consumer < Result < T , E > , Fut > for ResultVecConsumer < '_ , Fut , T , E > where Fut : Future < Output = Result < T , E > > , { type Output = () ; async fn send (self : Pin < & mut Self > , future : Fut) -> super :: ConsumerState { let mut this = self . project () ; this . group . as_mut () . push (future) ; ConsumerState :: Continue } async fn progress (self : Pin < & mut Self >) -> super :: ConsumerState { let mut this = self . project () ; let Ok (items) = this . output else { return ConsumerState :: Break ; } ; while let Some (item) = this . group . next () . await { match item { Ok (item) => { items . push (item) ; } Err (e) => { * * this . output = Err (e) ; return ConsumerState :: Break ; } } } ConsumerState :: Empty } async fn flush (self : Pin < & mut Self >) -> Self :: Output { self . progress () . await ; } }
    };
}

impl_152!();