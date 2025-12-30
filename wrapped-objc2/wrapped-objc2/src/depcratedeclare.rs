// Generated macro for declare (module)
macro_rules! Depcratedeclare {
() => {
// Module: crate
// Provides: {"declare"}
// Dependencies: {}
# [doc = " Deprecated location for a few things that are now in the [`runtime`]"] # [doc = " module."] # [deprecated = "Moved to the `runtime` module"] pub mod declare { use super :: runtime ; pub use super :: runtime :: { ClassBuilder , ProtocolBuilder } ; # [doc = " Use [`runtime::ClassBuilder`] instead."] # [deprecated = "Use `runtime::ClassBuilder` instead."] pub type ClassDecl = runtime :: ClassBuilder ; # [doc = " Use [`runtime::ProtocolBuilder`] instead."] # [deprecated = "Use `runtime::ProtocolBuilder` instead."] pub type ProtocolDecl = runtime :: ProtocolBuilder ; }
};
}
