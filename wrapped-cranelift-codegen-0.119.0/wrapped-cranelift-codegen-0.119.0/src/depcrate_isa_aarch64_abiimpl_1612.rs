// Generated macro for impl_1612 (impl)
macro_rules! Depcrate_isa_aarch64_abiimpl_1612 {
() => {
// Module: crate::isa::aarch64::abi
// Provides: {"impl_1612"}
// Dependencies: {}
impl AArch64CallSite { pub fn emit_return_call (mut self , ctx : & mut Lower < Inst > , args : isle :: ValueSlice , backend : & AArch64Backend ,) { let new_stack_arg_size = u32 :: try_from (self . sig (ctx . sigs ()) . sized_stack_arg_space ()) . unwrap () ; ctx . abi_mut () . accumulate_tail_args_size (new_stack_arg_size) ; self . emit_args (ctx , args) ; self . emit_stack_ret_arg_for_tail_call (ctx) ; let dest = self . dest () . clone () ; let uses = self . take_uses () ; let key = select_api_key (& backend . isa_flags , isa :: CallConv :: Tail , true) ; match dest { CallDest :: ExtName (callee , RelocDistance :: Near) => { let info = Box :: new (ReturnCallInfo { dest : callee , uses , key , new_stack_arg_size , }) ; ctx . emit (Inst :: ReturnCall { info }) ; } CallDest :: ExtName (name , RelocDistance :: Far) => { let callee = ctx . alloc_tmp (types :: I64) . only_reg () . unwrap () ; ctx . emit (Inst :: LoadExtName { rd : callee , name : Box :: new (name) , offset : 0 , }) ; let info = Box :: new (ReturnCallInfo { dest : callee . to_reg () , uses , key , new_stack_arg_size , }) ; ctx . emit (Inst :: ReturnCallInd { info }) ; } CallDest :: Reg (callee) => { let info = Box :: new (ReturnCallInfo { dest : callee , uses , key , new_stack_arg_size , }) ; ctx . emit (Inst :: ReturnCallInd { info }) ; } } } }
};
}
