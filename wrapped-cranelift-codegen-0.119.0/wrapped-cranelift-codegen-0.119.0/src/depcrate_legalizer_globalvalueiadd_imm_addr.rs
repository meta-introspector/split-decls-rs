// Generated macro for iadd_imm_addr (function)
macro_rules! Depcrate_legalizer_globalvalueiadd_imm_addr {
() => {
// Module: crate::legalizer::globalvalue
// Provides: {"iadd_imm_addr"}
// Dependencies: {}
# [doc = " Expand a `global_value` instruction for an iadd_imm global."] fn iadd_imm_addr (inst : ir :: Inst , func : & mut ir :: Function , base : ir :: GlobalValue , offset : i64 , global_type : ir :: Type ,) { let mut pos = FuncCursor :: new (func) . at_inst (inst) ; let lhs = pos . ins () . global_value (global_type , base) ; if let Some (fact) = & pos . func . global_value_facts [base] { pos . func . dfg . facts [lhs] = Some (fact . clone ()) ; } let constant = pos . ins () . iconst (global_type , offset) ; if pos . func . global_value_facts [base] . is_some () { let bits = u16 :: try_from (global_type . bits ()) . unwrap () ; let unsigned_offset = offset as u64 ; pos . func . dfg . facts [constant] = Some (Fact :: constant (bits , unsigned_offset)) ; } pos . func . dfg . replace (inst) . iadd (lhs , constant) ; }
};
}
