// Generated macro for NllRegionVariableOrigin (enum)
macro_rules! Depcrate_inferNllRegionVariableOrigin {
() => {
// Module: crate::infer
// Provides: {"NllRegionVariableOrigin"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] pub enum NllRegionVariableOrigin { # [doc = " During NLL region processing, we create variables for free"] # [doc = " regions that we encounter in the function signature and"] # [doc = " elsewhere. This origin indices we've got one of those."] FreeRegion , # [doc = " \"Universal\" instantiation of a higher-ranked region (e.g.,"] # [doc = " from a `for<'a> T` binder). Meant to represent \"any region\"."] Placeholder (ty :: PlaceholderRegion) , Existential { name : Option < Symbol > , } , }
};
}
