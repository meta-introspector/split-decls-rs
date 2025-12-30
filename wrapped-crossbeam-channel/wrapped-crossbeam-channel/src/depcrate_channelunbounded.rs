// Generated macro for unbounded (function)
macro_rules! Depcrate_channelunbounded {
() => {
// Module: crate::channel
// Provides: {"unbounded"}
// Dependencies: {}
# [doc = " Creates a multi-producer multi-consumer channel of unbounded capacity."] # [doc = ""] # [doc = " This channel has a growable buffer that can hold any number of messages at a time."] # [doc = ""] # [doc = " For more info on how to use the channel see [module level documentation](index.html)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::thread;"] # [doc = " use crossbeam_channel::unbounded;"] # [doc = ""] # [doc = " let (s, r) = unbounded();"] # [doc = ""] # [doc = " // Computes the n-th Fibonacci number."] # [doc = " fn fib(n: i32) -> i32 {"] # [doc = "     if n <= 1 {"] # [doc = "         n"] # [doc = "     } else {"] # [doc = "         fib(n - 1) + fib(n - 2)"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " // Spawn an asynchronous computation."] # [doc = " thread::spawn(move || s.send(fib(20)).unwrap());"] # [doc = ""] # [doc = " // Print the result of the computation."] # [doc = " println!(\"{}\", r.recv().unwrap());"] # [doc = " ```"] pub fn unbounded < T > () -> (Sender < T > , Receiver < T >) { let (s , r) = counter :: new (flavors :: list :: Channel :: new ()) ; let s = Sender { flavor : SenderFlavor :: List (s) , } ; let r = Receiver { flavor : ReceiverFlavor :: List (r) , } ; (s , r) }
};
}
