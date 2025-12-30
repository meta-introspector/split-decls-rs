// Generated macro for CargoError (struct)
macro_rules! Depcrate_errorCargoError {
() => {
// Module: crate::error
// Provides: {"CargoError"}
// Dependencies: {}
# [doc = " Cargo command failure information."] # [derive (Debug)] pub struct CargoError { kind : ErrorKind , context : Option < String > , cause : Option < Box < dyn Error + Send + Sync + 'static > > , }
};
}
