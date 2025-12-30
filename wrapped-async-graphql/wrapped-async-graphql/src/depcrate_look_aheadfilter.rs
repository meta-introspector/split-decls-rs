// Generated macro for filter (function)
macro_rules! Depcrate_look_aheadfilter {
() => {
// Module: crate::look_ahead
// Provides: {"filter"}
// Dependencies: {}
fn filter < 'a > (fields : & mut Vec < & 'a Field > , fragments : & 'a HashMap < Name , Positioned < FragmentDefinition > > , selection_set : & 'a SelectionSet , name : & str ,) { for item in & selection_set . items { match & item . node { Selection :: Field (field) => { if field . node . name . node == name { fields . push (& field . node) } } Selection :: InlineFragment (fragment) => { filter (fields , fragments , & fragment . node . selection_set . node , name) } Selection :: FragmentSpread (spread) => { if let Some (fragment) = fragments . get (& spread . node . fragment_name . node) { filter (fields , fragments , & fragment . node . selection_set . node , name) } } } } }
};
}
