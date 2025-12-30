// Generated macro for impl_2515 (impl)
macro_rules! Depcrate_isa_pulley_shared_abiimpl_2515 {
() => {
// Module: crate::isa::pulley_shared::abi
// Provides: {"impl_2515"}
// Dependencies: {}
impl < P > PulleyABICallSite < P > where P : PulleyTargetKind , { pub fn emit_return_call (mut self , ctx : & mut Lower < InstAndKind < P > > , args : isle :: ValueSlice , _backend : & PulleyBackend < P > ,) { let new_stack_arg_size = u32 :: try_from (self . sig (ctx . sigs ()) . sized_stack_arg_space ()) . unwrap () ; ctx . abi_mut () . accumulate_tail_args_size (new_stack_arg_size) ; self . emit_args (ctx , args) ; self . emit_stack_ret_arg_for_tail_call (ctx) ; let dest = self . dest () . clone () ; let uses = self . take_uses () ; match dest { CallDest :: ExtName (name , RelocDistance :: Near) => { let info = Box :: new (ReturnCallInfo { dest : name , uses , new_stack_arg_size , }) ; ctx . emit (Inst :: ReturnCall { info } . into ()) ; } CallDest :: ExtName (_name , RelocDistance :: Far) => { unimplemented ! ("return-call of a host function") } CallDest :: Reg (callee) => { let info = Box :: new (ReturnCallInfo { dest : XReg :: new (callee) . unwrap () , uses , new_stack_arg_size , }) ; ctx . emit (Inst :: ReturnIndirectCall { info } . into ()) ; } } } }
};
}
