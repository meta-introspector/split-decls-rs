// Generated macro for Substitution (type)
macro_rules! DepcrateSubstitution {
() => {
// Module: crate
// Provides: {"Substitution"}
// Dependencies: {}
# [doc = " Interned list of generic arguments for an item. When an item has parent, the `Substitution` for"] # [doc = " it contains generic arguments for both its parent and itself. See chalk's documentation for"] # [doc = " details."] # [doc = ""] # [doc = " See `Binders` for the constraint on the ordering."] pub type Substitution = chalk_ir :: Substitution < Interner > ;
};
}
