// Generated macro for private (module)
macro_rules! Depcrate_contextprivate {
() => {
// Module: crate::context
// Provides: {"private"}
// Dependencies: {}
pub (crate) mod private { use super :: * ; pub trait Sealed { } impl < T , E > Sealed for Result < T , E > where E : ext :: StdError { } impl < T > Sealed for Option < T > { } }
};
}
