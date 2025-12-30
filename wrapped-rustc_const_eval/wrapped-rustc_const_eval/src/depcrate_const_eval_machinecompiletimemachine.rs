// Generated macro for CompileTimeMachine (struct)
macro_rules! Depcrate_const_eval_machineCompileTimeMachine {
() => {
// Module: crate::const_eval::machine
// Provides: {"CompileTimeMachine"}
// Dependencies: {}
# [doc = " Extra machine state for CTFE, and the Machine instance."] pub struct CompileTimeMachine < 'tcx > { # [doc = " The number of terminators that have been evaluated."] # [doc = ""] # [doc = " This is used to produce lints informing the user that the compiler is not stuck."] # [doc = " Set to `usize::MAX` to never report anything."] pub (super) num_evaluated_steps : usize , # [doc = " The virtual call stack."] pub (super) stack : Vec < Frame < 'tcx > > , # [doc = " Pattern matching on consts with references would be unsound if those references"] # [doc = " could point to anything mutable. Therefore, when evaluating consts and when constructing valtrees,"] # [doc = " we ensure that only immutable global memory can be accessed."] pub (super) can_access_mut_global : CanAccessMutGlobal , # [doc = " Whether to check alignment during evaluation."] pub (super) check_alignment : CheckAlignment , # [doc = " If `Some`, we are evaluating the initializer of the static with the given `LocalDefId`,"] # [doc = " storing the result in the given `AllocId`."] # [doc = " Used to prevent accesses to a static's base allocation, as that may allow for self-initialization loops."] pub (crate) static_root_ids : Option < (AllocId , LocalDefId) > , # [doc = " A cache of \"data range\" computations for unions (i.e., the offsets of non-padding bytes)."] union_data_ranges : FxHashMap < Ty < 'tcx > , RangeSet > , }
};
}
