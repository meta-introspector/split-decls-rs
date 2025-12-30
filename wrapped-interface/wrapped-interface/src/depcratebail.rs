// Generated macro for bail (macro)
macro_rules! Depcratebail {
() => {
// Module: crate
// Provides: {"bail"}
// Dependencies: {}
macro_rules ! bail { ($ item : expr , $ ($ msg : tt) ,*) => { return Err (syn :: Error :: new ($ item . span () , std :: fmt :: format (format_args ! ($ ($ msg) ,*)))) ; } ; }
};
}
