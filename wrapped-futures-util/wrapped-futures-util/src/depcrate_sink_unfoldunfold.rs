// Generated macro for unfold (function)
macro_rules! Depcrate_sink_unfoldunfold {
() => {
// Module: crate::sink::unfold
// Provides: {"unfold"}
// Dependencies: {}
# [doc = " Create a sink from a function which processes one item at a time."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use core::pin::pin;"] # [doc = ""] # [doc = " use futures::sink;"] # [doc = " use futures::sink::SinkExt;"] # [doc = ""] # [doc = " let unfold = sink::unfold(0, |mut sum, i: i32| {"] # [doc = "     async move {"] # [doc = "         sum += i;"] # [doc = "         eprintln!(\"{}\", i);"] # [doc = "         Ok::<_, std::convert::Infallible>(sum)"] # [doc = "     }"] # [doc = " });"] # [doc = " let mut unfold = pin!(unfold);"] # [doc = " unfold.send(5).await?;"] # [doc = " # Ok::<(), std::convert::Infallible>(()) }).unwrap();"] # [doc = " ```"] pub fn unfold < T , F , Fut , Item , E > (init : T , function : F) -> Unfold < T , F , Fut > where F : FnMut (T , Item) -> Fut , Fut : Future < Output = Result < T , E > > , { assert_sink :: < Item , E , _ > (Unfold { function , state : UnfoldState :: Value { value : init } }) }
};
}
