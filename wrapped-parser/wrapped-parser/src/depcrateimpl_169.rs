// Generated macro for impl_169 (impl)
macro_rules! Depcrateimpl_169 {
() => {
// Module: crate
// Provides: {"impl_169"}
// Dependencies: {}
impl Error { # [doc = " Get an iterator over the positions of the error."] # [doc = ""] # [doc = " The iterator is ordered from most important to least important position."] # [must_use] pub fn positions (& self) -> ErrorPositions { match self { Self :: Syntax { start , end : Some (end) , .. } => ErrorPositions :: new_2 (* start , * end) , Self :: Syntax { start , .. } => ErrorPositions :: new_1 (* start) , Self :: MultipleRoots { schema , pos , .. } => ErrorPositions :: new_2 (* pos , * schema) , Self :: MissingQueryRoot { pos } => ErrorPositions :: new_1 (* pos) , Self :: MultipleOperations { anonymous , operation , } => ErrorPositions :: new_2 (* anonymous , * operation) , Self :: OperationDuplicated { first , second , .. } => { ErrorPositions :: new_2 (* second , * first) } Self :: FragmentDuplicated { first , second , .. } => { ErrorPositions :: new_2 (* second , * first) } Self :: MissingOperation => ErrorPositions :: new_0 () , Self :: RecursionLimitExceeded => ErrorPositions :: new_0 () , } } }
};
}
