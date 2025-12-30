// Generated macro for impl_1943 (impl)
macro_rules! Depcrate_isa_riscv64_abiimpl_1943 {
() => {
// Module: crate::isa::riscv64::abi
// Provides: {"impl_1943"}
// Dependencies: {}
impl Riscv64ABICallSite { pub fn emit_return_call (mut self , ctx : & mut Lower < Inst > , args : isle :: ValueSlice , _backend : & Riscv64Backend ,) { let new_stack_arg_size = u32 :: try_from (self . sig (ctx . sigs ()) . sized_stack_arg_space ()) . unwrap () ; ctx . abi_mut () . accumulate_tail_args_size (new_stack_arg_size) ; self . emit_args (ctx , args) ; self . emit_stack_ret_arg_for_tail_call (ctx) ; let dest = self . dest () . clone () ; let uses = self . take_uses () ; match dest { CallDest :: ExtName (name , RelocDistance :: Near) => { let info = Box :: new (ReturnCallInfo { dest : name , uses , new_stack_arg_size , }) ; ctx . emit (Inst :: ReturnCall { info }) ; } CallDest :: ExtName (name , RelocDistance :: Far) => { let callee = ctx . alloc_tmp (ir :: types :: I64) . only_reg () . unwrap () ; ctx . emit (Inst :: LoadExtName { rd : callee , name : Box :: new (name) , offset : 0 , }) ; let info = Box :: new (ReturnCallInfo { dest : callee . to_reg () , uses , new_stack_arg_size , }) ; ctx . emit (Inst :: ReturnCallInd { info }) ; } CallDest :: Reg (callee) => { let info = Box :: new (ReturnCallInfo { dest : callee , uses , new_stack_arg_size , }) ; ctx . emit (Inst :: ReturnCallInd { info }) ; } } } }
};
}
