// Generated macro for SampleBorrow (trait)
macro_rules! Depcrate_distr_uniformSampleBorrow {
() => {
// Module: crate::distr::uniform
// Provides: {"SampleBorrow"}
// Dependencies: {}
# [doc = " Helper trait similar to [`Borrow`] but implemented"] # [doc = " only for [`SampleUniform`] and references to [`SampleUniform`]"] # [doc = " in order to resolve ambiguity issues."] # [doc = ""] # [doc = " [`Borrow`]: std::borrow::Borrow"] pub trait SampleBorrow < Borrowed > { # [doc = " Immutably borrows from an owned value. See [`Borrow::borrow`]"] # [doc = ""] # [doc = " [`Borrow::borrow`]: std::borrow::Borrow::borrow"] fn borrow (& self) -> & Borrowed ; }
};
}
