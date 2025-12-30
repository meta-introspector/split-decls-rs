// Generated macro for Plot (trait)
macro_rules! Depcrate_traitsPlot {
() => {
// Module: crate::traits
// Provides: {"Plot"}
// Dependencies: {}
# [doc = " Overloaded `plot` method"] pub trait Plot < This > { # [doc = " The properties associated to the plot"] type Properties ; # [doc = " Plots some `data` with some `configuration`"] fn plot < F > (& mut self , this : This , function : F) -> & mut Self where F : FnOnce (& mut Self :: Properties) -> & mut Self :: Properties ; }
};
}
