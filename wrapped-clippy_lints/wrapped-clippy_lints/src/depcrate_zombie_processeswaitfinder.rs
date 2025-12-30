// Generated macro for WaitFinder (struct)
macro_rules! Depcrate_zombie_processesWaitFinder {
() => {
// Module: crate::zombie_processes
// Provides: {"WaitFinder"}
// Dependencies: {}
# [doc = " A visitor responsible for finding a `wait()` call on a local variable."] # [doc = ""] # [doc = " Note that this visitor does NOT explicitly look for `wait()` calls directly, but rather does the"] # [doc = " inverse -- checking if all uses of the local are either:"] # [doc = " - a field access (`child.{stderr,stdin,stdout}`)"] # [doc = " - calling `id` or `kill`"] # [doc = " - no use at all (e.g. `let _x = child;`)"] # [doc = " - taking a shared reference (`&`), `wait()` can't go through that"] # [doc = ""] # [doc = " None of these are sufficient to prevent zombie processes."] # [doc = " Doing it like this means more FNs, but FNs are better than FPs."] struct WaitFinder < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , local_id : HirId , create_id : HirId , body_id : LocalDefId , state : VisitorState , early_return : Option < Span > , missing_wait_branch : Option < MissingWaitBranch > , }
};
}
