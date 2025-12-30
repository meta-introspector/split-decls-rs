// Generated macro for FinalizeFn (type)
macro_rules! Depcrate_contextFinalizeFn {
() => {
// Module: crate::context
// Provides: {"FinalizeFn"}
// Dependencies: {}
type FinalizeFn < S > = Box < dyn Send + Sync + Fn (& mut FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > > ;
};
}
