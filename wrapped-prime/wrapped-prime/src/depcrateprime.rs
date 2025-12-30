// Generated macro for Prime (function)
macro_rules! DepcratePrime {
() => {
// Module: crate
// Provides: {"Prime"}
// Dependencies: {}
# [reactor] pub async fn Prime (mut scope : ReactorScope < ControlSignal , u64 >) { while let Some (m) = scope . next () . await { if m == ControlSignal :: Start { 'inner : for i in 1 .. { if primes :: is_prime (i) { scope . send (i) . await . unwrap () ; } futures :: select ! { m = scope . next () => { if m == Some (ControlSignal :: Stop) { break 'inner ; } } , _ = sleep (Duration :: from_millis (100)) . fuse () => { } , } } } } }
};
}
