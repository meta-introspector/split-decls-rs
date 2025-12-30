// Generated macro for impl_916 (impl)
macro_rules! Depcrate_ir_layoutimpl_916 {
() => {
// Module: crate::ir::layout
// Provides: {"impl_916"}
// Dependencies: {}
impl Layout { # [doc = " Compare the program points `a` and `b` in the same block relative to this program order."] # [doc = ""] # [doc = " Return `Less` if `a` appears in the program before `b`."] # [doc = ""] # [doc = " This is declared as a generic such that it can be called with `Inst` and `Block` arguments"] # [doc = " directly. Depending on the implementation, there is a good chance performance will be"] # [doc = " improved for those cases where the type of either argument is known statically."] pub fn pp_cmp < A , B > (& self , a : A , b : B) -> cmp :: Ordering where A : Into < ProgramPoint > , B : Into < ProgramPoint > , { let a = a . into () ; let b = b . into () ; debug_assert_eq ! (self . pp_block (a) , self . pp_block (b)) ; let a_seq = match a { ProgramPoint :: Block (_block) => 0 , ProgramPoint :: Inst (inst) => self . insts [inst] . seq , } ; let b_seq = match b { ProgramPoint :: Block (_block) => 0 , ProgramPoint :: Inst (inst) => self . insts [inst] . seq , } ; a_seq . cmp (& b_seq) } }
};
}
