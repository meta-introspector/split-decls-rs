macro_rules! deps {
    () => {
        FutureExt!();
        Send!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < Fut : Future > Fuse < Fut > { # [doc = " Creates a new `Fuse`-wrapped future which is already terminated."] # [doc = ""] # [doc = " This can be useful in combination with looping and the `select!`"] # [doc = " macro, which bypasses terminated futures."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use core::pin::pin;"] # [doc = ""] # [doc = " use futures::channel::mpsc;"] # [doc = " use futures::future::{Fuse, FusedFuture, FutureExt};"] # [doc = " use futures::select;"] # [doc = " use futures::stream::StreamExt;"] # [doc = ""] # [doc = " let (sender, mut stream) = mpsc::unbounded();"] # [doc = ""] # [doc = " // Send a few messages into the stream"] # [doc = " sender.unbounded_send(()).unwrap();"] # [doc = " sender.unbounded_send(()).unwrap();"] # [doc = " drop(sender);"] # [doc = ""] # [doc = " // Use `Fuse::terminated()` to create an already-terminated future"] # [doc = " // which may be instantiated later."] # [doc = " let foo_printer = Fuse::terminated();"] # [doc = " let mut foo_printer = pin!(foo_printer);"] # [doc = ""] # [doc = " loop {"] # [doc = "     select! {"] # [doc = "         _ = foo_printer => {},"] # [doc = "         () = stream.select_next_some() => {"] # [doc = "             if !foo_printer.is_terminated() {"] # [doc = "                 println!(\"Foo is already being printed!\");"] # [doc = "             } else {"] # [doc = "                 foo_printer.set(async {"] # [doc = "                     // do some other async operations"] # [doc = "                     println!(\"Printing foo from `foo_printer` future\");"] # [doc = "                 }.fuse());"] # [doc = "             }"] # [doc = "         },"] # [doc = "         complete => break, // `foo_printer` is terminated and the stream is done"] # [doc = "     }"] # [doc = " }"] # [doc = " # });"] # [doc = " ```"] pub fn terminated () -> Self { Self { inner : None } } }
    };
}

impl_48!();