// Generated macro for Configure (trait)
macro_rules! Depcrate_traitsConfigure {
() => {
// Module: crate::traits
// Provides: {"Configure"}
// Dependencies: {}
# [doc = " Overloaded `configure` method"] pub trait Configure < This > { # [doc = " The properties of what's being configured"] type Properties ; # [doc = " Configure some set of properties"] fn configure < F > (& mut self , this : This , function : F) -> & mut Self where F : FnOnce (& mut Self :: Properties) -> & mut Self :: Properties ; }
};
}
