// Generated macro for timeout (function)
macro_rules! Depcratetimeout {
() => {
// Module: crate
// Provides: {"timeout"}
// Dependencies: {}
async fn timeout < F : Future > (future_to_try : F , max_time : Duration ,) -> Result < F :: Output , Duration > { }
};
}
