// Generated macro for capstone_find_events (function)
macro_rules! Depcrate_shims_native_lib_trace_parentcapstone_find_events {
() => {
// Module: crate::shims::native_lib::trace::parent
// Provides: {"capstone_find_events"}
// Dependencies: {}
# [doc = " Add the memory events from `op` being executed while there is a memory access at `addr` to"] # [doc = " `acc_events`. Return whether this was a memory operand."] fn capstone_find_events (addr : usize , op : & capstone :: arch :: ArchOperand , acc_events : & mut Vec < AccessEvent > ,) -> bool { use capstone :: prelude :: * ; match op { # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] arch :: ArchOperand :: X86Operand (x86_operand) => { match x86_operand . op_type { arch :: x86 :: X86OperandType :: Mem (_) => { let push = AccessRange { addr , size : x86_operand . size . into () } ; let acc_ty = x86_operand . access . unwrap () ; if acc_ty . is_readable () { acc_events . push (AccessEvent :: Read (push . clone ())) ; } if acc_ty . is_writable () { acc_events . push (AccessEvent :: Write (push , ! acc_ty . is_readable ())) ; } return true ; } _ => () , } } _ => unimplemented ! () , } false }
};
}
