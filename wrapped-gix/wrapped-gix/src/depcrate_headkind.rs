// Generated macro for Kind (enum)
macro_rules! Depcrate_headKind {
() => {
// Module: crate::head
// Provides: {"Kind"}
// Dependencies: {}
# [doc = " Represents the kind of `HEAD` reference."] # [derive (Clone)] pub enum Kind { # [doc = " The existing reference the symbolic HEAD points to."] # [doc = ""] # [doc = " This is the common case."] Symbolic (gix_ref :: Reference) , # [doc = " The yet-to-be-created reference the symbolic HEAD refers to."] # [doc = ""] # [doc = " This is the case in a newly initialized repository."] Unborn (gix_ref :: FullName) , # [doc = " The head points to an object directly, not to a symbolic reference."] # [doc = ""] # [doc = " This state is less common and can occur when checking out commits directly."] Detached { # [doc = " The object to which the head points to"] target : ObjectId , # [doc = " Possibly the final destination of `target` after following the object chain from tag objects to commits."] peeled : Option < ObjectId > , } , }
};
}
