// Generated macro for impl_21 (impl)
macro_rules! Depcrate_commit_simpleimpl_21 {
() => {
// Module: crate::commit::simple
// Provides: {"impl_21"}
// Dependencies: {}
impl CommitState { pub fn is_hidden (& self) -> bool { matches ! (self , CommitState :: Hidden) } pub fn is_interesting (& self) -> bool { matches ! (self , CommitState :: Interesting) } }
};
}
