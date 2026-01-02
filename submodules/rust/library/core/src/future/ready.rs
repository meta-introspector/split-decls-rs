mkuse!{use crate :: future :: Future ;}
mkuse!{use crate :: pin :: Pin ;}
mkuse!{use crate :: task :: { Context , Poll } ;}
mkitem!{mkstruct!{# [doc = " A future that is immediately ready with a value."] # [doc = ""] # [doc = " This `struct` is created by [`ready()`]. See its"] # [doc = " documentation for more."] # [stable (feature = "future_readiness_fns" , since = "1.48.0")] # [derive (Debug , Clone)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Ready < T > (Option < T >) ;}}
mkitem!{mkimpl!{# [stable (feature = "future_readiness_fns" , since = "1.48.0")] impl < T > Unpin for Ready < T > { }}}
mkitem!{mkimpl!{# [stable (feature = "future_readiness_fns" , since = "1.48.0")] impl < T > Future for Ready < T > { type Output = T ; # [inline] fn poll (mut self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < T > { Poll :: Ready (self . 0 . take () . expect ("`Ready` polled after completion")) } }}}
mkitem!{mkimpl!{impl < T > Ready < T > { # [doc = " Consumes the `Ready`, returning the wrapped value."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Will panic if this [`Ready`] was already polled to completion."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::future;"] # [doc = ""] # [doc = " let a = future::ready(1);"] # [doc = " assert_eq!(a.into_inner(), 1);"] # [doc = " ```"] # [stable (feature = "ready_into_inner" , since = "1.82.0")] # [must_use] # [inline] pub fn into_inner (self) -> T { self . 0 . expect ("Called `into_inner()` on `Ready` after completion") } }}}

macro_rules! ready_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ready in module {}", module_path!());
    };
}

mkfn!{
    ready_introspect!();
    # [doc = " Creates a future that is immediately ready with a value."] # [doc = ""] # [doc = " Futures created through this function are functionally similar to those"] # [doc = " created through `async {}`. The main difference is that futures created"] # [doc = " through this function are named and implement `Unpin`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::future;"] # [doc = ""] # [doc = " # async fn run() {"] # [doc = " let a = future::ready(1);"] # [doc = " assert_eq!(a.await, 1);"] # [doc = " # }"] # [doc = " ```"] # [stable (feature = "future_readiness_fns" , since = "1.48.0")] pub fn ready < T > (t : T) -> Ready < T > { Ready (Some (t)) }
}