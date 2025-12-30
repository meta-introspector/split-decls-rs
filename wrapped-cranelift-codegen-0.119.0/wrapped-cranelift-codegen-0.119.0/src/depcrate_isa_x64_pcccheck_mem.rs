// Generated macro for check_mem (function)
macro_rules! Depcrate_isa_x64_pcccheck_mem {
() => {
// Module: crate::isa::x64::pcc
// Provides: {"check_mem"}
// Dependencies: {}
fn check_mem < 'a > (ctx : & FactContext , amode : & SyntheticAmode , vcode : & VCode < Inst > , ty : Type , op : LoadOrStore < 'a > ,) -> PccResult < Option < Fact > > { let addr = match amode { SyntheticAmode :: Real (amode) if amode . get_flags () . checked () => { compute_addr (ctx , vcode , amode , 64) . ok_or (PccError :: MissingFact) ? } _ => return Ok (None) , } ; match op { LoadOrStore :: Load { result_fact , from_bits , to_bits , } => { let loaded_fact = clamp_range (ctx , to_bits , from_bits , ctx . load (& addr , ty) ? . cloned ()) ? ; trace ! ("loaded_fact = {:?} result_fact = {:?}" , loaded_fact , result_fact) ; if ctx . subsumes_fact_optionals (loaded_fact . as_ref () , result_fact) { Ok (loaded_fact . clone ()) } else { Err (PccError :: UnsupportedFact) } } LoadOrStore :: Store { stored_fact } => { ctx . store (& addr , ty , stored_fact) ? ; Ok (None) } } }
};
}
