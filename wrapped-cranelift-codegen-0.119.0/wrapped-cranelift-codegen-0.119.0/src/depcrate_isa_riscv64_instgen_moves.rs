// Generated macro for gen_moves (function)
macro_rules! Depcrate_isa_riscv64_instgen_moves {
() => {
// Module: crate::isa::riscv64::inst
// Provides: {"gen_moves"}
// Dependencies: {}
# [doc = " rd and src must have the same length."] pub (crate) fn gen_moves (rd : & [Writable < Reg >] , src : & [Reg]) -> SmallInstVec < Inst > { assert ! (rd . len () == src . len ()) ; assert ! (rd . len () > 0) ; let mut insts = SmallInstVec :: new () ; for (dst , src) in rd . iter () . zip (src . iter ()) { let ty = Inst :: canonical_type_for_rc (dst . to_reg () . class ()) ; insts . push (Inst :: gen_move (* dst , * src , ty)) ; } insts }
};
}
