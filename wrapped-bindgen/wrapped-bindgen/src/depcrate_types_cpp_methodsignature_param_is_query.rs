// Generated macro for signature_param_is_query (function)
macro_rules! Depcrate_types_cpp_methodsignature_param_is_query {
() => {
// Module: crate::types::cpp_method
// Provides: {"signature_param_is_query"}
// Dependencies: {}
fn signature_param_is_query (params : & [Param]) -> Option < (usize , usize) > { if let Some (guid) = params . iter () . rposition (| param | param . ty == Type :: PtrConst (Box :: new (Type :: GUID) , 1) && param . is_input ()) { if let Some (object) = params . iter () . rposition (| param | { param . ty == Type :: PtrMut (Box :: new (Type :: Void) , 2) && param . def . has_attribute ("ComOutPtrAttribute") }) { return Some ((guid , object)) ; } } None }
};
}
