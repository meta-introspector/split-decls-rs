// Generated macro for HasCombination (trait)
macro_rules! Depcrate_adaptorsHasCombination {
() => {
// Module: crate::adaptors
// Provides: {"HasCombination"}
// Dependencies: {}
pub trait HasCombination < I > : Sized { type Combination : From < I > + Iterator < Item = Self > ; }
};
}
