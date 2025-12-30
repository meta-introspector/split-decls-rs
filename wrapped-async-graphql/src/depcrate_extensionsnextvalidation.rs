// Generated macro for NextValidation (struct)
macro_rules! Depcrate_extensionsNextValidation {
() => {
// Module: crate::extensions
// Provides: {"NextValidation"}
// Dependencies: {}
# [doc = " The remainder of a extension chain for validation."] pub struct NextValidation < 'a > { chain : & 'a [Arc < dyn Extension >] , validation_fut : ValidationFut < 'a > , }
};
}
