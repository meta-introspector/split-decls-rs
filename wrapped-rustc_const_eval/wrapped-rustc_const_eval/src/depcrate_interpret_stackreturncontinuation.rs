// Generated macro for ReturnContinuation (enum)
macro_rules! Depcrate_interpret_stackReturnContinuation {
() => {
// Module: crate::interpret::stack
// Provides: {"ReturnContinuation"}
// Dependencies: {}
# [doc = " Where and how to continue when returning/unwinding from the current function."] # [derive (Clone , Copy , Eq , PartialEq , Debug)] pub enum ReturnContinuation { # [doc = " Jump to the next block in the caller, or cause UB if None (that's a function"] # [doc = " that may never return)."] # [doc = " `ret` stores the block we jump to on a normal return, while `unwind`"] # [doc = " stores the block used for cleanup during unwinding."] Goto { ret : Option < mir :: BasicBlock > , unwind : mir :: UnwindAction } , # [doc = " The root frame of the stack: nowhere else to jump to, so we stop."] # [doc = " `cleanup` says whether locals are deallocated. Static computation"] # [doc = " wants them leaked to intern what they need (and just throw away"] # [doc = " the entire `ecx` when it is done)."] Stop { cleanup : bool } , }
};
}
