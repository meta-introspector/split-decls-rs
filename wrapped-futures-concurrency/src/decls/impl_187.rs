macro_rules! deps {
    () => {
        Consumer!();
        Try!();
        ConsumerState!();
        TryForEachFut!();
        TryForEachConsumer!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < FutT , T , F , FutB , B > Consumer < T , FutT > for TryForEachConsumer < FutT , T , F , FutB , B > where FutT : Future < Output = T > , F : Clone + Fn (T) -> FutB , FutB : Future < Output = B > , B : Try < Output = () > , { type Output = B ; async fn send (self : Pin < & mut Self > , future : FutT) -> super :: ConsumerState { let mut this = self . project () ; while this . count . load (Ordering :: Relaxed) >= * this . limit { match this . group . next () . await { None => break , Some (res) => match res . branch () { ControlFlow :: Continue (_) => continue , ControlFlow :: Break (residual) => { * this . residual = Some (residual) ; return ConsumerState :: Break ; } } , } } this . count . fetch_add (1 , Ordering :: Relaxed) ; let fut = TryForEachFut :: new (this . f . clone () , future , this . count . clone ()) ; this . group . as_mut () . push (fut) ; ConsumerState :: Continue } async fn progress (self : Pin < & mut Self >) -> super :: ConsumerState { let mut this = self . project () ; while let Some (res) = this . group . next () . await { if let ControlFlow :: Break (residual) = res . branch () { * this . residual = Some (residual) ; return ConsumerState :: Break ; } } ConsumerState :: Empty } async fn flush (self : Pin < & mut Self >) -> Self :: Output { let mut this = self . project () ; if this . residual . is_some () { return B :: from_residual (this . residual . take () . unwrap ()) ; } while let Some (res) = this . group . next () . await { if let ControlFlow :: Break (residual) = res . branch () { return B :: from_residual (residual) ; } } B :: from_output (()) } }
    };
}

impl_187!()