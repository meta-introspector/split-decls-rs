// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use gloo :: worker :: Spawnable ; use wasm_bindgen_test :: * ; wasm_bindgen_test_configure ! (run_in_browser) ; # [wasm_bindgen_test] async fn prime_worker_works () { gloo :: console :: log ! ("running test") ; let mut bridge = Prime :: spawner () . spawn ("http://127.0.0.1:9999/example_prime_worker.js") ; bridge . send (ControlSignal :: Start) . await . expect ("failed to send start signal") ; sleep (Duration :: from_millis (1050)) . await ; bridge . send (ControlSignal :: Stop) . await . expect ("failed to send stop signal") ; let primes : Vec < _ > = bridge . take (5) . collect () . await ; assert_eq ! (primes , vec ! [2 , 3 , 5 , 7 , 11]) ; } }
};
}
