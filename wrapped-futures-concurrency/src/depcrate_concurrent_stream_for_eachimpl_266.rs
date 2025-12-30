// Generated macro for impl_266 (impl)
macro_rules! Depcrate_concurrent_stream_for_eachimpl_266 {
() => {
// Module: crate::concurrent_stream::for_each
// Provides: {"impl_266"}
// Dependencies: {}
impl < FutT , T , F , B > Consumer < T , FutT > for ForEachConsumer < FutT , T , F , B > where FutT : Future < Output = T > , F : Fn (T) -> B , F : Clone , B : Future < Output = () > , { type Output = () ; async fn send (self : Pin < & mut Self > , future : FutT) -> super :: ConsumerState { let mut this = self . project () ; while this . count . load (Ordering :: Relaxed) >= * this . limit { this . group . next () . await ; } this . count . fetch_add (1 , Ordering :: Relaxed) ; let fut = ForEachFut :: new (this . f . clone () , future , this . count . clone ()) ; this . group . as_mut () . push (fut) ; ConsumerState :: Continue } async fn progress (self : Pin < & mut Self >) -> super :: ConsumerState { let mut this = self . project () ; while (this . group . next () . await) . is_some () { } ConsumerState :: Empty } async fn flush (self : Pin < & mut Self >) -> Self :: Output { let mut this = self . project () ; while (this . group . next () . await) . is_some () { } } }
};
}
