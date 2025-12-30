// Generated macro for expand_global_value (function)
macro_rules! Depcrate_legalizer_globalvalueexpand_global_value {
() => {
// Module: crate::legalizer::globalvalue
// Provides: {"expand_global_value"}
// Dependencies: {}
# [doc = " Expand a `global_value` instruction according to the definition of the global value."] pub fn expand_global_value (inst : ir :: Inst , func : & mut ir :: Function , isa : & dyn TargetIsa , global_value : ir :: GlobalValue ,) { crate :: trace ! ("expanding global value: {:?}: {}" , inst , func . dfg . display_inst (inst)) ; match func . global_values [global_value] { ir :: GlobalValueData :: VMContext => vmctx_addr (global_value , inst , func) , ir :: GlobalValueData :: IAddImm { base , offset , global_type , } => iadd_imm_addr (inst , func , base , offset . into () , global_type) , ir :: GlobalValueData :: Load { base , offset , global_type , flags , } => load_addr (inst , func , base , offset , global_type , flags , isa) , ir :: GlobalValueData :: Symbol { tls , .. } => symbol (inst , func , global_value , isa , tls) , ir :: GlobalValueData :: DynScaleTargetConst { vector_type } => { const_vector_scale (inst , func , vector_type , isa) } } }
};
}
