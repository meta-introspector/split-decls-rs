// Generated macro for RebaseOperation (struct)
macro_rules! Depcrate_rebaseRebaseOperation {
() => {
// Module: crate::rebase
// Provides: {"RebaseOperation"}
// Dependencies: {}
# [doc = " A rebase operation"] # [doc = ""] # [doc = " Describes a single instruction/operation to be performed during the"] # [doc = " rebase."] # [derive (Debug)] pub struct RebaseOperation < 'rebase > { raw : * const raw :: git_rebase_operation , _marker : marker :: PhantomData < Rebase < 'rebase > > , }
};
}
