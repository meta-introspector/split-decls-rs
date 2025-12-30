// Generated macro for symbol (function)
macro_rules! Depcrate_legalizer_globalvaluesymbol {
() => {
// Module: crate::legalizer::globalvalue
// Provides: {"symbol"}
// Dependencies: {}
# [doc = " Expand a `global_value` instruction for a symbolic name global."] fn symbol (inst : ir :: Inst , func : & mut ir :: Function , gv : ir :: GlobalValue , isa : & dyn TargetIsa , tls : bool ,) { let ptr_ty = isa . pointer_type () ; if tls { func . dfg . replace (inst) . tls_value (ptr_ty , gv) ; } else { func . dfg . replace (inst) . symbol_value (ptr_ty , gv) ; } }
};
}
