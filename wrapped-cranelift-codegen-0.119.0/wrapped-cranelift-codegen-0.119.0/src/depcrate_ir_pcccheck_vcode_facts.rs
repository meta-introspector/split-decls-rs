// Generated macro for check_vcode_facts (function)
macro_rules! Depcrate_ir_pcccheck_vcode_facts {
() => {
// Module: crate::ir::pcc
// Provides: {"check_vcode_facts"}
// Dependencies: {}
# [doc = " Top-level entry point after compilation: this checks the facts in"] # [doc = " VCode."] pub fn check_vcode_facts < B : LowerBackend + TargetIsa > (f : & ir :: Function , vcode : & mut VCode < B :: MInst > , backend : & B ,) -> PccResult < () > { let ctx = FactContext :: new (f , backend . triple () . pointer_width () . unwrap () . bits () . into ()) ; for block in 0 .. vcode . num_blocks () { let block = BlockIndex :: new (block) ; let mut flow_state = B :: FactFlowState :: default () ; for inst in vcode . block_insns (block) . iter () { if let Err (e) = backend . check_fact (& ctx , vcode , inst , & mut flow_state) { log :: info ! ("Error checking instruction: {:?}" , vcode [inst]) ; return Err (e) ; } if vcode . is_branch (inst) { for (succ_idx , succ) in vcode . block_succs (block) . iter () . enumerate () { for (arg , param) in vcode . branch_blockparams (block , inst , succ_idx) . iter () . zip (vcode . block_params (* succ) . iter ()) { let arg_fact = vcode . vreg_fact (* arg) ; let param_fact = vcode . vreg_fact (* param) ; if ! ctx . subsumes_fact_optionals (arg_fact , param_fact) { return Err (PccError :: UnsupportedBlockparam) ; } } } } } } Ok (()) }
};
}
