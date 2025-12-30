// Generated macro for continue_on_interrupt (function)
macro_rules! Depcrate_inputcontinue_on_interrupt {
() => {
// Module: crate::input
// Provides: {"continue_on_interrupt"}
// Dependencies: {}
fn continue_on_interrupt < T > (result : Result < T , std :: io :: Error >) -> Action < T > { match result { Ok (v) => Action :: Result (Ok (v)) , Err (err) if err . kind () == std :: io :: ErrorKind :: Interrupted => Action :: Continue , Err (err) => Action :: Result (Err (err)) , } }
};
}
