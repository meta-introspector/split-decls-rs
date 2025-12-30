// Generated macro for remove_from_parent (function)
macro_rules! Depcrate_rcdomremove_from_parent {
() => {
// Module: crate::rcdom
// Provides: {"remove_from_parent"}
// Dependencies: {}
fn remove_from_parent (target : & Handle) { { let (parent , i) = unwrap_or_return ! (get_parent_and_index (target) , ()) ; parent . borrow_mut () . children . remove (i) ; } let mut child = target . borrow_mut () ; (* child) . parent = None ; }
};
}
