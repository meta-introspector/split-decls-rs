// Generated macro for require_class (function)
macro_rules! Depcrate_jsrequire_class {
() => {
// Module: crate::js
// Provides: {"require_class"}
// Dependencies: {}
fn require_class < 'a > (exported_classes : & 'a mut Option < BTreeMap < String , ExportedClass > > , name : & str ,) -> & 'a mut ExportedClass { exported_classes . as_mut () . expect ("classes already written") . entry (name . to_string ()) . or_default () }
};
}
