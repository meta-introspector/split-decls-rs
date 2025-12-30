// Generated macro for OptionSet (trait)
macro_rules! DepcrateOptionSet {
() => {
// Module: crate
// Provides: {"OptionSet"}
// Dependencies: {}
# [doc = " Trait for bit flags that forwards to std traits for useful bit operators."] pub trait OptionSet : Copy + Default + Eq + BitAnd < Output = Self > + BitOrAssign + 'static { # [doc = " The basis flags (in the algebraic sense): one for each independent option."] const VARIANTS : & 'static [Self] ; # [doc = " The corresponding names. `VARIANTS.len() == NAMES.len()` must always hold."] const NAMES : & 'static [& 'static str] ; }
};
}
