// Generated macro for put_input_in_regs (function)
macro_rules! Depcrate_isa_x64_lowerput_input_in_regs {
() => {
// Module: crate::isa::x64::lower
// Provides: {"put_input_in_regs"}
// Dependencies: {}
# [doc = " Put the given input into possibly multiple registers, and mark it as used (side-effect)."] fn put_input_in_regs (ctx : & mut Lower < Inst > , spec : InsnInput) -> ValueRegs < Reg > { let ty = ctx . input_ty (spec . insn , spec . input) ; let input = ctx . get_input_as_source_or_const (spec . insn , spec . input) ; if let Some (c) = input . constant { let size = if ty_bits (ty) < 64 { OperandSize :: Size32 } else { OperandSize :: Size64 } ; assert ! (is_int_or_ref_ty (ty)) ; let cst_copy = ctx . alloc_tmp (ty) ; ctx . emit (Inst :: imm (size , c , cst_copy . only_reg () . unwrap ())) ; non_writable_value_regs (cst_copy) } else { ctx . put_input_in_regs (spec . insn , spec . input) } }
};
}
