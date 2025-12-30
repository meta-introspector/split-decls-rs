// Generated macro for ValidationFut (type)
macro_rules! Depcrate_extensionsValidationFut {
() => {
// Module: crate::extensions
// Provides: {"ValidationFut"}
// Dependencies: {}
type ValidationFut < 'a > = & 'a mut (dyn Future < Output = Result < ValidationResult , Vec < ServerError > > > + Send + Unpin) ;
};
}
