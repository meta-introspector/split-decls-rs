// Generated macro for can_exec (function)
macro_rules! Depcrate_dfacan_exec {
() => {
// Module: crate::dfa
// Provides: {"can_exec"}
// Dependencies: {}
# [doc = " Return true if and only if the given program can be executed by a DFA."] # [doc = ""] # [doc = " Generally, a DFA is always possible. A pathological case where it is not"] # [doc = " possible is if the number of NFA states exceeds u32::MAX, in which case,"] # [doc = " this function will return false."] # [doc = ""] # [doc = " This function will also return false if the given program has any Unicode"] # [doc = " instructions (Char or Ranges) since the DFA operates on bytes only."] pub fn can_exec (insts : & Program) -> bool { use prog :: Inst :: * ; if insts . len () > :: std :: i32 :: MAX as usize { return false ; } for inst in insts { match * inst { Char (_) | Ranges (_) => return false , EmptyLook (_) | Match (_) | Save (_) | Split (_) | Bytes (_) => { } } } true }
};
}
