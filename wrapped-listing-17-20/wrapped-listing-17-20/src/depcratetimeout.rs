// Generated macro for timeout (function)
macro_rules! Depcratetimeout {
() => {
// Module: crate
// Provides: {"timeout"}
// Dependencies: {}
async fn timeout < F : Future > (future_to_try : F , max_time : Duration ,) -> Result < F :: Output , Duration > { match trpl :: select (future_to_try , trpl :: sleep (max_time)) . await { Either :: Left (output) => Ok (output) , Either :: Right (_) => Err (max_time) , } }
};
}
