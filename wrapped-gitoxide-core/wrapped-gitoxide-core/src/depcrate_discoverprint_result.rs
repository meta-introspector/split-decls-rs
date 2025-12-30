// Generated macro for print_result (function)
macro_rules! Depcrate_discoverprint_result {
() => {
// Module: crate::discover
// Provides: {"print_result"}
// Dependencies: {}
fn print_result < T , E > (mut out : impl std :: io :: Write , res : Result < T , E >) -> std :: io :: Result < bool > where T : std :: fmt :: Debug , E : std :: error :: Error + Send + Sync + 'static , { let mut has_err = false ; let to_print = match res { Ok (good) => { format ! ("{good:#?}") } Err (err) => { has_err = true ; format ! ("{:?}" , anyhow :: Error :: from (err)) } } ; indent (& mut out , to_print) ? ; Ok (has_err) }
};
}
