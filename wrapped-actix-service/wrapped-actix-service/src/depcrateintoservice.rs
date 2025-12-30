// Generated macro for IntoService (trait)
macro_rules! DepcrateIntoService {
() => {
// Module: crate
// Provides: {"IntoService"}
// Dependencies: {}
# [doc = " Trait for types that can be converted to a `Service`"] pub trait IntoService < S , Req > where S : Service < Req > , { # [doc = " Convert to a `Service`"] fn into_service (self) -> S ; }
};
}
