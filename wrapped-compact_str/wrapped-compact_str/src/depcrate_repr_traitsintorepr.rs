// Generated macro for IntoRepr (trait)
macro_rules! Depcrate_repr_traitsIntoRepr {
() => {
// Module: crate::repr::traits
// Provides: {"IntoRepr"}
// Dependencies: {}
# [doc = " Defines how to _efficiently_ create a [`Repr`] from `self`"] pub (crate) trait IntoRepr { fn into_repr (self) -> Result < Repr , ToCompactStringError > ; }
};
}
