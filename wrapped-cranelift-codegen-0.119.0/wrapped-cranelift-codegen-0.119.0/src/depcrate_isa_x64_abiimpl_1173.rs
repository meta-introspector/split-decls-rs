// Generated macro for impl_1173 (impl)
macro_rules! Depcrate_isa_x64_abiimpl_1173 {
() => {
// Module: crate::isa::x64::abi
// Provides: {"impl_1173"}
// Dependencies: {}
impl X64CallSite { pub fn emit_return_call (mut self , ctx : & mut Lower < Inst > , args : isle :: ValueSlice , _backend : & X64Backend ,) { let new_stack_arg_size = u32 :: try_from (self . sig (ctx . sigs ()) . sized_stack_arg_space ()) . unwrap () ; ctx . abi_mut () . accumulate_tail_args_size (new_stack_arg_size) ; self . emit_args (ctx , args) ; self . emit_stack_ret_arg_for_tail_call (ctx) ; let dest = self . dest () . clone () ; let uses = self . take_uses () ; let tmp = ctx . temp_writable_gpr () ; match dest { CallDest :: ExtName (callee , RelocDistance :: Near) => { let info = Box :: new (ReturnCallInfo { dest : callee , uses , tmp , new_stack_arg_size , }) ; ctx . emit (Inst :: ReturnCallKnown { info }) ; } CallDest :: ExtName (callee , RelocDistance :: Far) => { let tmp2 = ctx . temp_writable_gpr () ; ctx . emit (Inst :: LoadExtName { dst : tmp2 . to_writable_reg () , name : Box :: new (callee) , offset : 0 , distance : RelocDistance :: Far , }) ; let info = Box :: new (ReturnCallInfo { dest : tmp2 . to_reg () . to_reg () . into () , uses , tmp , new_stack_arg_size , }) ; ctx . emit (Inst :: ReturnCallUnknown { info }) ; } CallDest :: Reg (callee) => { let info = Box :: new (ReturnCallInfo { dest : callee . into () , uses , tmp , new_stack_arg_size , }) ; ctx . emit (Inst :: ReturnCallUnknown { info }) ; } } } }
};
}
