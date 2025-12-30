// Generated macro for append (function)
macro_rules! Depcrate_rcdomappend {
() => {
// Module: crate::rcdom
// Provides: {"append"}
// Dependencies: {}
fn append (new_parent : & Handle , child : Handle) { new_parent . borrow_mut () . children . push (child . clone ()) ; let parent = & mut child . borrow_mut () . parent ; assert ! (parent . is_none ()) ; * parent = Some (Rc :: downgrade (new_parent)) ; }
};
}
