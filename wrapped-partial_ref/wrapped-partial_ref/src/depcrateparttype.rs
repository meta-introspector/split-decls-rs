// Generated macro for PartType (trait)
macro_rules! DepcratePartType {
() => {
// Module: crate
// Provides: {"PartType"}
// Dependencies: {}
# [doc = " Type of a part, determines what can be done with a part."] # [doc = ""] # [doc = " Common part types are [`Field`] and [`AbstractPart`]."] pub trait PartType { # [doc = " Type that can be produced from a constant pointer to a reference target."] type Ptr ; # [doc = " Type that can be produced from a mutable pointer to a reference target."] type PtrMut ; }
};
}
