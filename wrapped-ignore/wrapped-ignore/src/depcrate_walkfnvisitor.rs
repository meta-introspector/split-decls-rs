// Generated macro for FnVisitor (type)
macro_rules! Depcrate_walkFnVisitor {
() => {
// Module: crate::walk
// Provides: {"FnVisitor"}
// Dependencies: {}
type FnVisitor < 's > = Box < dyn FnMut (Result < DirEntry , Error >) -> WalkState + Send + 's > ;
};
}
