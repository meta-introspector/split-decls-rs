// Generated macro for const_vector_scale (function)
macro_rules! Depcrate_legalizer_globalvalueconst_vector_scale {
() => {
// Module: crate::legalizer::globalvalue
// Provides: {"const_vector_scale"}
// Dependencies: {}
fn const_vector_scale (inst : ir :: Inst , func : & mut ir :: Function , ty : ir :: Type , isa : & dyn TargetIsa) { assert ! (ty . bytes () <= 16) ; let base_bytes = std :: cmp :: max (ty . bytes () , 16) ; let scale = (isa . dynamic_vector_bytes (ty) / base_bytes) as i64 ; assert ! (scale > 0) ; let pos = FuncCursor :: new (func) . at_inst (inst) ; pos . func . dfg . replace (inst) . iconst (isa . pointer_type () , scale) ; }
};
}
