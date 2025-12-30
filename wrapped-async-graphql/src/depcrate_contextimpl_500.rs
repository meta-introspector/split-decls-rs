// Generated macro for impl_500 (impl)
macro_rules! Depcrate_contextimpl_500 {
() => {
// Module: crate::context
// Provides: {"impl_500"}
// Dependencies: {}
impl < 'a > Iterator for SelectionFieldsIter < 'a > { type Item = SelectionField < 'a > ; fn next (& mut self) -> Option < Self :: Item > { loop { let it = self . iter . last_mut () ? ; let item = it . next () ; match item { Some (selection) => match & selection . node { Selection :: Field (field) => { return Some (SelectionField { fragments : self . fragments , field : & field . node , context : self . context , }) ; } Selection :: FragmentSpread (fragment_spread) => { if let Some (fragment) = self . fragments . get (& fragment_spread . node . fragment_name . node) { self . iter . push (fragment . node . selection_set . node . items . iter ()) ; } } Selection :: InlineFragment (inline_fragment) => { self . iter . push (inline_fragment . node . selection_set . node . items . iter ()) ; } } , None => { self . iter . pop () ; } } } } }
};
}
