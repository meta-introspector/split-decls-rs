// Generated macro for get_parent_and_index (function)
macro_rules! Depcrate_rcdomget_parent_and_index {
() => {
// Module: crate::rcdom
// Provides: {"get_parent_and_index"}
// Dependencies: {}
fn get_parent_and_index (target : & Handle) -> Option < (Handle , usize) > { let child = target . borrow () ; let parent = unwrap_or_return ! (child . parent . as_ref () , None) . upgrade () . expect ("dangling weak pointer") ; let i = match parent . borrow_mut () . children . iter () . enumerate () . find (| & (_ , n) | same_node (n , target)) { Some ((i , _)) => i , None => panic ! ("have parent but couldn't find in parent's children!") , } ; Some ((Handle (parent) , i)) }
};
}
