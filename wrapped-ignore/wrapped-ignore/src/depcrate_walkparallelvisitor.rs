// Generated macro for ParallelVisitor (trait)
macro_rules! Depcrate_walkParallelVisitor {
() => {
// Module: crate::walk
// Provides: {"ParallelVisitor"}
// Dependencies: {}
# [doc = " Receives files and directories for the current thread."] # [doc = ""] # [doc = " Setup for the traversal can be implemented as part of"] # [doc = " [`ParallelVisitorBuilder::build`]. Teardown when traversal finishes can be"] # [doc = " implemented by implementing the `Drop` trait on your traversal type."] pub trait ParallelVisitor : Send { # [doc = " Receives files and directories for the current thread. This is called"] # [doc = " once for every directory entry visited by traversal."] fn visit (& mut self , entry : Result < DirEntry , Error >) -> WalkState ; }
};
}
