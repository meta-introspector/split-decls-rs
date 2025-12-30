// Generated macro for LifetimeKind (enum)
macro_rules! Depcrate_contextLifetimeKind {
() => {
// Module: crate::context
// Provides: {"LifetimeKind"}
// Dependencies: {}
# [doc = " The kind of lifetime we are completing."] # [derive (Debug)] pub (crate) enum LifetimeKind { LifetimeParam , Lifetime { in_lifetime_param_bound : bool , def : Option < hir :: GenericDef > } , LabelRef , LabelDef , }
};
}
