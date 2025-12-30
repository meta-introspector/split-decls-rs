// Generated macro for Coalesce (type)
macro_rules! Depcrate_adaptors_coalesceCoalesce {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"Coalesce"}
// Dependencies: {}
# [doc = " An iterator adaptor that may join together adjacent elements."] # [doc = ""] # [doc = " See [`.coalesce()`](crate::Itertools::coalesce) for more information."] pub type Coalesce < I , F > = CoalesceBy < I , F , NoCount > ;
};
}
