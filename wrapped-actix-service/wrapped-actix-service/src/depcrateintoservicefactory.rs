// Generated macro for IntoServiceFactory (trait)
macro_rules! DepcrateIntoServiceFactory {
() => {
// Module: crate
// Provides: {"IntoServiceFactory"}
// Dependencies: {}
# [doc = " Trait for types that can be converted to a `ServiceFactory`"] pub trait IntoServiceFactory < SF , Req > where SF : ServiceFactory < Req > , { # [doc = " Convert `Self` to a `ServiceFactory`"] fn into_factory (self) -> SF ; }
};
}
