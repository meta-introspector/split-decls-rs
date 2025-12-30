// Generated macro for append_to_existing_text (function)
macro_rules! Depcrate_rcdomappend_to_existing_text {
() => {
// Module: crate::rcdom
// Provides: {"append_to_existing_text"}
// Dependencies: {}
fn append_to_existing_text (prev : & Handle , text : & str) -> bool { match prev . borrow_mut () . deref_mut () . node { Text (ref mut existing) => { existing . push_slice (text) ; true } _ => false , } }
};
}
