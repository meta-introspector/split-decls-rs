use serde::{Deserialize, Serialize};
use std::collections::HashMap;
macro_rules! delegate_all {
    (@ trait Future $name:ident < $($arg:ident),* > ($t:ty) $(where $($bound:tt)*)*) => {
        impl <$($arg),*> futures_core::future::Future for $name <$($arg),*> where $t :
        futures_core::future::Future $(, $($bound)*)* { type Output = <$t as
        futures_core::future::Future >::Output; delegate_future!(inner); }
    };
    (
        @ trait FusedFuture $name:ident < $($arg:ident),* > ($t:ty) $(where
        $($bound:tt)*)*
    ) => {
        impl <$($arg),*> futures_core::future::FusedFuture for $name <$($arg),*> where $t
        : futures_core::future::FusedFuture $(, $($bound)*)* { fn is_terminated(& self)
        -> bool { self.inner.is_terminated() } }
    };
    (@ trait Stream $name:ident < $($arg:ident),* > ($t:ty) $(where $($bound:tt)*)*) => {
        impl <$($arg),*> futures_core::stream::Stream for $name <$($arg),*> where $t :
        futures_core::stream::Stream $(, $($bound)*)* { type Item = <$t as
        futures_core::stream::Stream >::Item; delegate_stream!(inner); }
    };
    (
        @ trait FusedStream $name:ident < $($arg:ident),* > ($t:ty) $(where
        $($bound:tt)*)*
    ) => {
        impl <$($arg),*> futures_core::stream::FusedStream for $name <$($arg),*> where $t
        : futures_core::stream::FusedStream $(, $($bound)*)* { fn is_terminated(& self)
        -> bool { self.inner.is_terminated() } }
    };
    (@ trait Sink $name:ident < $($arg:ident),* > ($t:ty) $(where $($bound:tt)*)*) => {
        #[cfg(feature = "sink")] impl < _Item, $($arg),*> futures_sink::Sink < _Item >
        for $name <$($arg),*> where $t : futures_sink::Sink < _Item > $(, $($bound)*)* {
        type Error = <$t as futures_sink::Sink < _Item >>::Error; delegate_sink!(inner,
        _Item); }
    };
    (@ trait Debug $name:ident < $($arg:ident),* > ($t:ty) $(where $($bound:tt)*)*) => {
        impl <$($arg),*> core::fmt::Debug for $name <$($arg),*> where $t :
        core::fmt::Debug $(, $($bound)*)* { fn fmt(& self, f : & mut core::fmt::Formatter
        <'_ >) -> core::fmt::Result { core::fmt::Debug::fmt(& self.inner, f) } }
    };
    (
        @ trait AccessInner[$inner:ty, ($($ind:tt)*)] $name:ident < $($arg:ident),* >
        ($t:ty) $(where $($bound:tt)*)*
    ) => {
        impl <$($arg),*> $name <$($arg),*> $(where $($bound)*)* {
        delegate_access_inner!(inner, $inner, ($($ind)*)); }
    };
    (
        @ trait New[|$($param:ident : $paramt:ty),*| $cons:expr] $name:ident <
        $($arg:ident),* > ($t:ty) $(where $($bound:tt)*)*
    ) => {
        impl <$($arg),*> $name <$($arg),*> $(where $($bound)*)* { pub (crate) fn
        new($($param : $paramt),*) -> Self { Self { inner : $cons } } }
    };
    (
        $(#[$attr:meta])* $name:ident <$($arg:ident),*> ($t:ty) : $ftrait:ident
        $([$($targs:tt)*])* $({ $($item:tt)* })* $(where $($bound:tt)*)*
    ) => {
        pin_project_lite::pin_project! { #[must_use =
        "futures/streams/sinks do nothing unless you `.await` or poll them"] $(#[$attr])*
        pub struct $name < $($arg),* > $(where $($bound)*)* { #[pin] inner : $t } } impl
        <$($arg),*> $name < $($arg),* > $(where $($bound)*)* { $($($item)*)* }
        delegate_all!(@ trait $ftrait $([$($targs)*])* $name <$($arg),*> ($t) $(where
        $($bound)*)*);
    };
    (
        $(#[$attr:meta])* $name:ident <$($arg:ident),*> ($t:ty) : $ftrait:ident
        $([$($ftargs:tt)*])* + $strait:ident $([$($stargs:tt)*])* $(+ $trait:ident
        $([$($targs:tt)*])*)* $({ $($item:tt)* })* $(where $($bound:tt)*)*
    ) => {
        delegate_all!($(#[$attr])* $name <$($arg),*> ($t) : $strait $([$($stargs)*])* $(+
        $trait $([$($targs)*])*)* $({ $($item)* })* $(where $($bound)*)*);
        delegate_all!(@ trait $ftrait $([$($ftargs)*])* $name <$($arg),*> ($t) $(where
        $($bound)*)*);
    };
}
