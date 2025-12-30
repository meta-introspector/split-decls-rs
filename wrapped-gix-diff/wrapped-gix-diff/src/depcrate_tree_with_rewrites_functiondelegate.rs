// Generated macro for Delegate (struct)
macro_rules! Depcrate_tree_with_rewrites_functionDelegate {
() => {
// Module: crate::tree_with_rewrites::function
// Provides: {"Delegate"}
// Dependencies: {}
struct Delegate < 'a , 'old , VisitFn , E , Objects > { src_tree : TreeRefIter < 'old > , recorder : crate :: tree :: Recorder , objects : & 'a Objects , visit : VisitFn , tracked : Option < rewrites :: Tracker < crate :: tree :: visit :: Change > > , location : Option < crate :: tree :: recorder :: Location > , err : Option < E > , }
};
}
