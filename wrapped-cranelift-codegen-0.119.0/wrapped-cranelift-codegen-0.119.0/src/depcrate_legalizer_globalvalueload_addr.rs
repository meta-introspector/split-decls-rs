// Generated macro for load_addr (function)
macro_rules! Depcrate_legalizer_globalvalueload_addr {
() => {
// Module: crate::legalizer::globalvalue
// Provides: {"load_addr"}
// Dependencies: {}
# [doc = " Expand a `global_value` instruction for a load global."] fn load_addr (inst : ir :: Inst , func : & mut ir :: Function , base : ir :: GlobalValue , offset : ir :: immediates :: Offset32 , global_type : ir :: Type , flags : ir :: MemFlags , isa : & dyn TargetIsa ,) { let ptr_ty = isa . pointer_type () ; let mut pos = FuncCursor :: new (func) . at_inst (inst) ; pos . use_srcloc (inst) ; let base_addr = pos . ins () . global_value (ptr_ty , base) ; if let Some (fact) = & pos . func . global_value_facts [base] { pos . func . dfg . facts [base_addr] = Some (fact . clone ()) ; } pos . func . dfg . replace (inst) . load (global_type , flags , base_addr , offset) ; }
};
}
