macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! ready {
    () => {
        deps!();
        # [doc = " Unwraps `Poll<T>` or returns [`Pending`][`core::task::Poll::Pending`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::{future, prelude::*, ready};"] # [doc = " use std::pin::Pin;"] # [doc = " use std::task::{Context, Poll};"] # [doc = ""] # [doc = " fn do_poll(cx: &mut Context<'_>) -> Poll<()> {"] # [doc = "     let mut fut = future::ready(42);"] # [doc = "     let fut = Pin::new(&mut fut);"] # [doc = ""] # [doc = "     let num = ready!(fut.poll(cx));"] # [doc = "     # drop(num);"] # [doc = "     // ... use num"] # [doc = ""] # [doc = "     Poll::Ready(())"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " The `ready!` call expands to:"] # [doc = ""] # [doc = " ```"] # [doc = " # use futures_lite::{future, prelude::*, ready};"] # [doc = " # use std::pin::Pin;"] # [doc = " # use std::task::{Context, Poll};"] # [doc = " #"] # [doc = " # fn do_poll(cx: &mut Context<'_>) -> Poll<()> {"] # [doc = "     # let mut fut = future::ready(42);"] # [doc = "     # let fut = Pin::new(&mut fut);"] # [doc = "     #"] # [doc = " let num = match fut.poll(cx) {"] # [doc = "     Poll::Ready(t) => t,"] # [doc = "     Poll::Pending => return Poll::Pending,"] # [doc = " };"] # [doc = "     # drop(num);"] # [doc = "     # // ... use num"] # [doc = "     #"] # [doc = "     # Poll::Ready(())"] # [doc = " # }"] # [doc = " ```"] # [macro_export] macro_rules ! ready { ($ e : expr $ (,) ?) => { match $ e { core :: task :: Poll :: Ready (t) => t , core :: task :: Poll :: Pending => return core :: task :: Poll :: Pending , } } ; }
    };
}

ready!()