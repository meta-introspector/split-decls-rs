// Generated macro for GroupTypeInner (struct)
macro_rules! Depcrate_contextGroupTypeInner {
() => {
// Module: crate::context
// Provides: {"GroupTypeInner"}
// Dependencies: {}
pub (super) struct GroupTypeInner < S : Stage > { pub (super) accepters : BTreeMap < & 'static [Symbol] , Vec < GroupTypeInnerAccept < S > > > , pub (super) finalizers : Vec < FinalizeFn < S > > , }
};
}
