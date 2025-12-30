// Generated macro for SelectionField (struct)
macro_rules! Depcrate_contextSelectionField {
() => {
// Module: crate::context
// Provides: {"SelectionField"}
// Dependencies: {}
# [doc = " Selection field."] # [derive (Clone , Copy)] pub struct SelectionField < 'a > { pub (crate) fragments : & 'a HashMap < Name , Positioned < FragmentDefinition > > , pub (crate) field : & 'a Field , pub (crate) context : & 'a Context < 'a > , }
};
}
