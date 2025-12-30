// Generated macro for BreakableContext (struct)
macro_rules! Depcrate_inferBreakableContext {
() => {
// Module: crate::infer
// Provides: {"BreakableContext"}
// Dependencies: {}
# [derive (Clone , Debug)] struct BreakableContext < 'db > { # [doc = " Whether this context contains at least one break expression."] may_break : bool , # [doc = " The coercion target of the context."] coerce : Option < DynamicCoerceMany < 'db > > , # [doc = " The optional label of the context."] label : Option < LabelId > , kind : BreakableKind , }
};
}
