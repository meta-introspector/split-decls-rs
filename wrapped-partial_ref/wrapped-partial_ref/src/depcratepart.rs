// Generated macro for Part (trait)
macro_rules! DepcratePart {
() => {
// Module: crate
// Provides: {"Part"}
// Dependencies: {}
# [doc = " Marker types for a part of a type."] # [doc = ""] # [doc = " Types implementing this trait are usually created using the [`part`] macro."] # [doc = ""] # [doc = " A type implementing this trait is used to identify a part of a reference target. Multiple"] # [doc = " different reference targets can have a part identified by the same [`Part`] (see also"] # [doc = " [`HasPart`]). A part has an associated [`PartType`], which determines what can be done with a"] # [doc = " part."] pub trait Part : Default { type PartType : PartType ; }
};
}
