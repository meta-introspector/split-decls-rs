macro_rules! pin {
    () => {
        # [doc = " Pins a variable of type `T` on the stack and rebinds it as `Pin<&mut T>`."] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::{future, pin};"] # [doc = " use std::fmt::Debug;"] # [doc = " use std::future::Future;"] # [doc = " use std::pin::Pin;"] # [doc = " use std::time::Instant;"] # [doc = ""] # [doc = " // Inspects each invocation of `Future::poll()`."] # [doc = " async fn inspect<T: Debug>(f: impl Future<Output = T>) -> T {"] # [doc = "     pin!(f);"] # [doc = "     future::poll_fn(|cx| dbg!(f.as_mut().poll(cx))).await"] # [doc = " }"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let f = async { 1 + 2 };"] # [doc = " inspect(f).await;"] # [doc = " # })"] # [doc = " ```"] # [macro_export] macro_rules ! pin { ($ ($ x : ident) ,* $ (,) ?) => { $ (let mut $ x = :: core :: pin :: pin ! ($ x) ;) * } }
    };
}

pin!();